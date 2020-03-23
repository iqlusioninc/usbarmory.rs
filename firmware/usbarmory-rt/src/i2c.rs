use pac::{CCM, I2C1, IOMUXC};

use crate::rtc;

// in 32.768 KHz "ticks"
const TIMEOUT: u32 = 33; // one millisecond

const RNW_WRITE: u8 = 0;
const RNW_READ: u8 = 1;

const I2CR_MTX: u16 = 1 << 4;

const I2SR_IBB: u16 = 1 << 5;
const I2SR_IIF: u16 = 1 << 1;

pub fn init() {
    // configure I2C1 pins
    IOMUXC::borrow_unchecked(|iomuxc| unsafe {
        // SCL
        const SW_MUX_CTL_PAD_GPIO1_IO02: *mut u32 = 0x20E_0064 as *mut u32;
        SW_MUX_CTL_PAD_GPIO1_IO02.write_volatile(0x10);

        const SW_PAD_CTL_PAD_GPIO1_IO02: *mut u32 = 0x20E_02F0 as *mut u32;
        SW_PAD_CTL_PAD_GPIO1_IO02.write_volatile(0x0001_b8b1);

        iomuxc.I2C1_SCL_SELECT_INPUT.write(0);

        // SDA
        const SW_MUX_CTL_PAD_GPIO1_IO03: *mut u32 = 0x20E_0068 as *mut u32;
        SW_MUX_CTL_PAD_GPIO1_IO03.write_volatile(0x10);

        const SW_PAD_CTL_PAD_GPIO1_IO03: *mut u32 = 0x20E_02F4 as *mut u32;
        SW_PAD_CTL_PAD_GPIO1_IO03.write_volatile(0x0001_b8b1);

        iomuxc.I2C1_SDA_SELECT_INPUT.write(1);
    });

    CCM::borrow_unchecked(|ccm| {
        // ungate I2C1's clock
        ccm.CCGR2.rmw(|ccgr| ccgr | (0b11 << 6));
    });

    I2C1::borrow_unchecked(|i2c| {
        // I2C frequency = 66 MHz / 768 ~= 86 KHz < 100 KHz
        i2c.IFDR.write(0x39);

        // reset the I2C until we need it
        i2c.I2CR.write(0);
    });
}

pub fn init_fusb303() -> Result<(), Error> {
    const DEVADDR: u8 = 0x31;
    const REGADDR: u8 = 0x05;
    const VAL: u8 = 0xbb; // reset value (0xb3) w/ the ENABLE bit set

    i2c_write(DEVADDR, REGADDR, VAL)?;

    // sanity check
    if i2c_write_then_read(DEVADDR, REGADDR)? == VAL {
        Ok(())
    } else {
        Err(Error)
    }
}

pub struct Error;

// START - DEVADDR+W - REGADDR - reSTART - DEVADDR+R - <byte> - STOP
fn i2c_write_then_read(devaddr: u8, regaddr: u8) -> Result<u8, Error> {
    i2c_start()?;

    for byte in [(devaddr << 1) | RNW_WRITE, regaddr].iter() {
        i2c_tx(*byte)?;
    }

    i2c_restart();

    // NOTE("software restriction") wait ~82 CPU clock cycles after setting RSTA and before
    // writing I2DR (see section 29.6 of 6ULRM)
    for _ in 0..41 {
        cortex_a::nop();
    }

    // send slave address + READ bit
    i2c_tx((devaddr << 1) | RNW_READ)?;

    I2C1::borrow_unchecked(|i2c| {
        const I2CR_TXAK: u16 = 1 << 3;

        // switch to read mode & NAK the next incoming byte
        i2c.I2CR.rmw(|i2cr| (i2cr & !I2CR_MTX) | I2CR_TXAK);
        // dummy read to start the transfer
        i2c.I2DR.read();

        // wait for the transfer to complete
        i2c_wait_for_sr(I2SR_IIF, State::Set)?;

        i2c_stop()?;

        Ok(i2c.I2DR.read() as u8)
    })
}

// START - DEVADDR+W - REGADDR - VAL - STOP
fn i2c_write(devaddr: u8, regaddr: u8, val: u8) -> Result<(), Error> {
    i2c_start()?;

    for byte in [(devaddr << 1) | RNW_WRITE, regaddr, val].iter() {
        i2c_tx(*byte)?;
    }

    i2c_stop()
}

const I2CR_MSTA: u16 = 1 << 5;

fn i2c_start() -> Result<(), Error> {
    const I2CR_IEN: u16 = 1 << 7;

    I2C1::borrow_unchecked(|i2c| {
        // clear status register
        i2c.I2SR.write(0);

        // enable I2C
        i2c.I2CR.write(I2CR_IEN);

        // while until the bus is free (should succeed immediately)
        i2c_wait_for_sr(I2SR_IBB, State::Clear)?;

        // send START
        i2c.I2CR.rmw(|i2cr| i2cr | I2CR_MSTA | I2CR_MTX);

        // while until we gain the bus
        i2c_wait_for_sr(I2SR_IBB, State::Set)?;

        Ok(())
    })
}

fn i2c_restart() {
    I2C1::borrow_unchecked(|i2c| {
        const I2CR_RSTA: u16 = 1 << 2;

        // send reSTART
        i2c.I2CR.rmw(|i2cr| i2cr | I2CR_RSTA);
    });
}

fn i2c_stop() -> Result<(), Error> {
    I2C1::borrow_unchecked(|i2c| {
        // send STOP signal
        i2c.I2CR.rmw(|i2cr| (i2cr & !I2CR_MSTA) & !I2CR_MTX);

        // wait until the bus is free
        i2c_wait_for_sr(I2SR_IBB, State::Clear)?;

        Ok(())
    })
}

fn i2c_tx(byte: u8) -> Result<(), Error> {
    const I2SR_RXAK: u16 = 1; // << 0

    I2C1::borrow_unchecked(|i2c| {
        // send byte
        i2c.I2DR.write(u16::from(byte));

        // wait until transfer is complete
        let sr = i2c_wait_for_sr(I2SR_IIF, State::Set)?;

        if sr & I2SR_RXAK != 0 {
            // ACK error!
            return Err(Error);
        }

        Ok(())
    })
}

fn i2c_wait_for_sr(flag: u16, state: State) -> Result<u16, Error> {
    const I2SR_IAL: u16 = 1 << 4;

    I2C1::borrow_unchecked(|i2c| {
        // wait until transfer is complete
        let mut sr;
        let start = rtc::now();
        loop {
            sr = i2c.I2SR.read();

            if sr & I2SR_IAL != 0 {
                // arbitration lost!
                return Err(Error);
            }

            let expected = match state {
                State::Set => flag,
                State::Clear => 0,
            };

            if sr & flag == expected {
                break;
            }

            if rtc::now() >= start + TIMEOUT {
                return Err(Error);
            }
        }

        if flag == I2SR_IIF && state == State::Set {
            i2c.I2SR.write(0);
        }

        Ok(sr)
    })
}

/// Flag state
#[derive(PartialEq)]
enum State {
    Set,
    Clear,
}
