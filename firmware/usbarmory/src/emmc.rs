//! Low level access to the eMMC

use core::{
    num::NonZeroU16,
    sync::atomic::{self, Ordering},
};

use pac::{uSDHC2, SRC};

mod card;
mod cmd;

use core::{fmt, time::Duration};

use crate::{
    memlog, memlog_flush_and_reset,
    storage::{Block, ManagedBlockDevice, BLOCK_SIZE},
    time::{self, Instant},
    util,
};
use cmd::Command;

fn default_timeout() -> Duration {
    Duration::from_millis(100)
}

/// [Singleton] Access to the on-board eMMC
#[allow(non_camel_case_types)]
pub struct eMMC {
    usdhc: uSDHC2,
    /// Currently selected card
    selected: Option<Rca>,
    blocks: u32,
    verbose: bool,
    width: Width,
}

#[derive(Clone, Copy)]
enum Width {
    /// 1-bit
    B1,
    /// 8-bit
    B8,
    // modes not listed: 4-bit, 4-bit DDR
}

impl Width {
    fn dtw(self) -> u8 {
        match self {
            Width::B1 => 0b00,
            Width::B8 => 0b10,
        }
    }

    fn ddr(self) -> bool {
        false
    }
}

type Rca = NonZeroU16;

/// Command complete
const INT_STATUS_CC: u32 = 1; // bit 0
/// Transfer complete
const INT_STATUS_TC: u32 = 1 << 1;
/// DMA Interrupt
const INT_STATUS_DINT: u32 = 1 << 3;
/// Command Timeout Error
const INT_STATUS_CTOE: u32 = 1 << 16;
/// Command CRC Error
const INT_STATUS_CCE: u32 = 1 << 17;
/// Command End Bit Error
const INT_STATUS_CEBE: u32 = 1 << 18;
/// Command Index Error
const INT_STATUS_CIE: u32 = 1 << 19;
/// Data Timeout Error
const INT_STATUS_DTOE: u32 = 1 << 20;
/// Data CRC Error
const INT_STATUS_DCE: u32 = 1 << 21;
/// Data End Bit Error
const INT_STATUS_DEBE: u32 = 1 << 22;
/// DMA error
const INT_STATUS_DMAE: u32 = 1 << 28;

const INT_STATUS_ANY_ERROR: u32 = INT_STATUS_CTOE
    | INT_STATUS_CCE
    | INT_STATUS_CEBE
    | INT_STATUS_CIE
    | INT_STATUS_DTOE
    | INT_STATUS_DCE
    | INT_STATUS_DEBE
    | INT_STATUS_DMAE;

/// Command Inhibit (DATA)
const PRES_STATE_CDIHB: u32 = 1 << 1;
/// Command Inhibit
const PRES_STATE_CIHB: u32 = 1; // bit 0
/// Data Line Active
const PRES_STATE_DLA: u32 = 1 << 2;

/// Bits that must always be ones
const MIX_CTRL_RESERVED: u32 = 1 << 31;
/// Single block
const MIX_CTRL_MBSEL_SINGLE: u32 = 0 << 5;
/// Enable the DMA
const MIX_CTRL_DMAEN_ENABLE: u32 = 1; // bit 0
const MIX_CTRL_DDR_EN: u32 = 1 << 3;

const VEND_SPEC_CKEN: u32 = 1 << 14;

/// Relative address assigned to the eMMC (by the ROM bootloader)
// must be greater than the default (0x01)
const RCA: Rca = unsafe { Rca::new_unchecked(0x02) };

#[derive(Debug)]
enum Frequency {
    K400,
    M20,
}

impl eMMC {
    /// Gets a handle to the `eMMC` singleton
    ///
    /// This method returns the `Some` only once. When successful this method
    /// consumes the `uSDHC2` peripheral
    pub fn take() -> Option<Result<Self, Error>> {
        uSDHC2::take().map(|usdhc| {
            // boot mode register
            let sbmr1 = SRC::borrow_unchecked(|src| src.SBMR1.read());

            // booted from the uSD
            if sbmr1 & 0b1110_0000 == 0b0100_0000 {
                panic!("the eMMC HAL doesn't work when booting from the uSD");
            }

            // clear any pending status
            usdhc.INT_STATUS.clear(!0);
            // we'll *not* use the advanced DMA (ADMA)
            usdhc.PROT_CTRL.rmw(|r| r & !(0b11 << 8));

            // NOTE the ROM bootloader has already configured the card to run at high speed and to
            // work with an 8-bit bus
            let mut emmc = eMMC {
                usdhc,
                blocks: 0,
                selected: Some(RCA),
                verbose: false,
                width: Width::B8,
            };

            emmc.software_reset();
            emmc.reset_cards().expect("fatal");
            emmc.voltage_validation().expect("fatal");
            emmc.register_cards().expect("fatal");
            emmc.select_card(RCA).expect("fatal");
            emmc.change_bus_width(Width::B8, true).expect("fatal");
            emmc.change_frequency(Frequency::M20);

            let mut ext_csd = [0; 512];
            emmc.read_single_block(None, ext_csd.as_mut_ptr())
                .expect("fatal");

            emmc.blocks =
                u32::from_le_bytes([ext_csd[212], ext_csd[213], ext_csd[214], ext_csd[215]]);

            memlog!("card has {} blocks", emmc.blocks);

            Ok(emmc)
        })
    }

    /// Changes the verbosity of the driver (default: false)
    pub fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Gets a handle to the `eMMC` singleton
    ///
    /// This method returns the `Some` only once. When successful this method
    /// consumes the `uSDHC2` peripheral
    // this implementation ought to support both the "eMMC" and "uSD" boot modes but it doesn't
    // work ATM
    #[cfg(TODO = "eMMC_and_uSD_support")]
    pub fn take() -> Option<Result<Self, Error>> {
        uSDHC2::take().map(|usdhc| {
            let mut emmc = Self {
                usdhc,
                blocks: 0,
                selected: None,
                verbose: false,
                width: Width::B1,
            };

            emmc.software_reset();
            emmc.reset_cards();
            emmc.voltage_validation();
            emmc.register_cards()?;
            let csd = emmc.identify_card(RCA);

            assert_eq!(csd.version(), 4, "only eMMC v4.x is supported");
            for block_size in &[csd.max_read_block_size(), csd.max_write_block_size()] {
                assert!(
                    *block_size >= BLOCK_SIZE,
                    "unsupported block size ({})",
                    block_size
                );
            }
            emmc.select_card(RCA)?;

            let mut ext_csd = [0; 512];
            emmc.read_single_block(None, ext_csd.as_mut_ptr())
                .expect("EXT_CSD read failed");

            let card_type = ext_csd[196];
            assert_ne!(
                card_type & 2,
                0,
                "card doesn't support up to 52 MHz frequencies"
            );

            emmc.blocks =
                u32::from_le_bytes([ext_csd[212], ext_csd[213], ext_csd[214], ext_csd[215]]);

            emmc.send_command(Command::Switch { data: 0x3B9_0100 }, false);
            emmc.wait_response()?;
            card::Status::from(emmc.usdhc.CMD_RSP0.read())?;

            // wait for the busy line to clear
            emmc.get_card_status(RCA, false)?;

            memlog!("put card in high speed mode @ {:?}", time::uptime(),);

            // // TODO change bus width to DDR 4-bit mode
            // emmc.send_command(Command::Switch { data: 0x3B70100 }, false);
            // emmc.wait_response().unwrap();
            // let status1 = card::Status::from(emmc.usdhc.CMD_RSP0.read()).unwrap();

            // // wait for the busy line to clear
            // let status2 = emmc.get_card_status(RCA, false).unwrap();

            // emmc.ddr = true;

            // memlog!(
            //     "changed card's bus width @ {:?} ({:?}, {:?})",
            //     time::uptime(),
            //     status1,
            //     status2
            // );

            // emmc.usdhc.SYS_CTRL.rmw(|mut r| {
            //     // lowest 4 bits must always be ones
            //     r |= 0xf;

            //     const DTW_MASK: u32 = 0b11 << 1;
            //     const DTW_B4: u32 = 0b01 << 1;
            //     // const DTW_B8: u32 = 0b10 << 1;

            //     r = (r & !DTW_MASK) | DTW_B4;

            //     memlog!("> SYS_CTRL: {:#010x}", r);
            //     r
            // });

            emmc.change_frequency(Frequency::M48);

            Ok(emmc)
        })
    }

    // TODO partial reads and write (if the card supports them)
    /// Reads a block of memory
    pub fn read(&self, block_nr: u32, block: &mut Block) -> Result<(), Error> {
        assert!(block_nr < self.blocks, "block doesn't exist");

        if self.verbose {
            memlog!("read(block_nr={} @ {:?}", block_nr, time::uptime());
        }

        self.read_single_block(Some(block_nr), block.bytes.as_mut_ptr())
    }

    /// Transfers a block of memory to the card for it to be programmed to flash
    pub fn write(&self, block_nr: u32, block: &Block) -> Result<(), Error> {
        assert!(block_nr < self.blocks, "block doesn't exist");

        if self.verbose {
            memlog!("write(block_nr={}) @ {:?}", block_nr, time::uptime());
        }

        self.write_single_block(block_nr, block.bytes.as_ptr())
    }

    // mid-level API
    // NOTE(`block_nr`) this assumes that the card is a high-capacity device
    // NOTE `block_nr=None` reads the EXT_CSD register
    fn read_single_block(&self, block_nr: Option<u32>, addr: *mut u8) -> Result<(), Error> {
        let rca = if let Some(rca) = self.selected {
            rca
        } else {
            memlog!("a card must be selected first");
            memlog_flush_and_reset!();
        };

        debug_assert_eq!(addr as usize % 4, 0);

        let status = self.get_card_status(rca)?;
        if self.verbose {
            memlog!("{:?}", status);
        }

        if !status.ready_for_data || status.state != card::State::Transfer {
            return Err(Error::NotInTransferState);
        }

        /// Data Transfer Direction Select = Read (Card to Host)
        const MIX_CTRL_DTDSEL_READ: u32 = 1 << 4;
        self.usdhc.MIX_CTRL.write(
            MIX_CTRL_RESERVED
                | if self.width.ddr() { MIX_CTRL_DDR_EN } else { 0 }
                | MIX_CTRL_MBSEL_SINGLE
                | MIX_CTRL_DTDSEL_READ
                | MIX_CTRL_DMAEN_ENABLE,
        );

        // must wait until DLA is cleared
        while self.usdhc.PRES_STATE.read() & PRES_STATE_DLA != 0 {
            crate::memlog_try_flush()
        }

        // TC must be cleared before writing to DS_ADDR
        self.usdhc.INT_STATUS.clear(INT_STATUS_TC);

        // semantically, the next register write transfers ownership of the
        // buffer to the DMA so ensure all previous memory operations complete
        // before the ownership transfer (NOTE this could be done a bit later:
        // in the middle of `send_command(WriteSingleBlock)`)
        atomic::fence(Ordering::Release);
        self.usdhc.DS_ADDR.write(addr as usize as u32);
        self.usdhc.BLK_ATT.write(1 << 16 | u32::from(BLOCK_SIZE));
        self.usdhc.WTMK_LVL.reset(); // use a reasonable watermark level

        // start the transfer
        let cmd = block_nr
            .map(|block_nr| Command::ReadSingleBlock { block_nr })
            .unwrap_or(Command::SendExtCsd);
        if self.verbose {
            memlog!("read START @ {:?}", time::uptime());
        }
        let rsp = self.send_command(cmd)?;
        let status = card::Status::from(rsp)?;

        // NOTE `send_command` will block until the data transfer is finished

        // TODO accessing `buf` requires cache invalidation
        // let the DMA finish its memory operations before `block` is read
        atomic::fence(Ordering::Acquire);

        if self.verbose {
            memlog!("read DONE ({:?}) @ {:?}", status, time::uptime());
        }
        Ok(())
    }

    fn write_single_block(&self, block_nr: u32, addr: *const u8) -> Result<(), Error> {
        let rca = if let Some(rca) = self.selected {
            rca
        } else {
            memlog!("a card must be selected first");
            memlog_flush_and_reset!();
        };

        debug_assert_eq!(addr as usize % 4, 0);

        let status = self.get_card_status(rca)?;
        if !status.ready_for_data || status.state != card::State::Transfer {
            if self.verbose {
                memlog!("error: {:?}", status);
            }
            return Err(Error::NotInTransferState);
        }

        /// Data Transfer Direction Select = Write (Host to Card)
        const MIX_CTRL_DTDSEL_WRITE: u32 = 0 << 4;
        self.usdhc.MIX_CTRL.write(
            MIX_CTRL_RESERVED
                | if self.width.ddr() { MIX_CTRL_DDR_EN } else { 0 }
                | MIX_CTRL_MBSEL_SINGLE
                | MIX_CTRL_DTDSEL_WRITE
                | MIX_CTRL_DMAEN_ENABLE,
        );

        // must wait until DLA is cleared
        while self.usdhc.PRES_STATE.read() & PRES_STATE_DLA != 0 {
            crate::memlog_try_flush()
        }

        // TC must be cleared before writing to DS_ADDR
        self.usdhc.INT_STATUS.clear(INT_STATUS_TC);

        // semantically, the next register write transfers ownership of the
        // buffer to the DMA so ensure all previous memory operations complete
        // before the ownership transfer (NOTE this could be done a bit later:
        // in the middle of `send_command(WriteSingleBlock)`)
        atomic::fence(Ordering::Release);
        self.usdhc.DS_ADDR.write(addr as usize as u32);
        self.usdhc.BLK_ATT.write(1 << 16 | u32::from(BLOCK_SIZE));
        self.usdhc.WTMK_LVL.reset(); // use a reasonable watermark level

        // start the transfer
        if self.verbose {
            memlog!("write START @ {:?}", time::uptime());
        }
        let rsp = self.send_command(Command::WriteSingleBlock { block_nr })?;
        let status = card::Status::from(rsp)?;

        // NOTE `send_command` will block until the data transfer is finished

        // buffer handled back to us
        atomic::fence(Ordering::Acquire);

        // flush the write
        if status.state == card::State::Programming {
            // wait until the card finishes programming the data it received
            let start = Instant::now();
            while self.get_card_status(rca)?.state != card::State::Transfer {
                crate::memlog_try_flush();

                if Instant::now() - start > default_timeout() {
                    memlog!("card took too long to program to flash the data it received");
                    memlog_flush_and_reset!();
                }
            }

            memlog!("write flushed @ {:?}", time::uptime());
        }

        if self.verbose {
            memlog!("write DONE @ {:?}", time::uptime());
        }

        Ok(())
    }

    /// "Selects" the card with the specific relative address `rca`
    ///
    /// A card must be selected before commands like `read_single_block` and
    /// `write_single_block` can be used
    fn select_card(&mut self, rca: Rca) -> Result<(), Error> {
        // a more general implementation would check this address against the
        // ones registered in `register_cards`
        assert_eq!(rca, RCA);

        // select the card
        self.send_command(Command::SelectCard { rca: Some(rca) })?;

        // set block size (card side)
        self.send_command(Command::SetBlockLen {
            len: BLOCK_SIZE.into(),
        })?;

        self.selected = Some(rca);

        Ok(())
    }

    /// Returns the card specific data of the card with the specified relative
    /// address
    #[cfg(TODO = "eMMC_and_uSD_support")]
    fn identify_card(&self, rca: Rca) -> card::Csd {
        // a more general implementation would check this address against the
        // ones registered in `register_cards`
        assert_eq!(rca, RCA);

        self.send_command(Command::SendCsd { rca }, true);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }

        card::Csd::from([
            self.usdhc.CMD_RSP0.read(),
            self.usdhc.CMD_RSP1.read(),
            self.usdhc.CMD_RSP2.read(),
            self.usdhc.CMD_RSP3.read(),
        ])
    }

    /// Registers (gives them a relative address) all cards on the bus
    fn register_cards(&self) -> Result<(), Error> {
        // NOTE here we assume that only the eMMC is connected to this uSDHC bus
        // if there were more cards on the bus then this should be a loop that
        // assigns a different relative address to each one
        memlog!("registering cards on the bus");
        let cid0 = self.send_command(Command::AllSendCid)?;

        let _cid = [
            cid0,
            self.usdhc.CMD_RSP1.read(),
            self.usdhc.CMD_RSP2.read(),
            self.usdhc.CMD_RSP3.read(),
        ];

        self.send_command(Command::SetRelativeAddr { rca: RCA })?;
        memlog!("registered a card with RCA={:#04x}", RCA);

        // we omit this because we know there's only one card
        // // this should fail with a timeout because there no more cards on the
        // // bus
        // self.send_command(Command::AllSendCid, true);
        // let res = self.wait_response();

        // if res != Err(Error::Timeout) {
        //     memlog!("expected command timeout but got {:?}", res);
        //     memlog_flush_and_reset!();
        // }

        // self.clear_command_inhibit();

        // memlog!("no more cards on the bus");

        Ok(())
    }

    /// Selects an operating voltage that supports most (all?) cards on the bus
    fn voltage_validation(&self) -> Result<(), Error> {
        let target_ocr = 0x00FF_8000;

        /// 0 = Card is busy; 1 = Card has finished powering up
        const OCR_RDY: u32 = 1 << 31;

        // broadcast operating voltage; wait until cards have finished powering up
        let start = Instant::now();
        let ocr = loop {
            let ocr = self.send_command(Command::SendOpCond { ocr: target_ocr })?;

            if ocr & OCR_RDY != 0 {
                break ocr;
            }

            if start.elapsed() > default_timeout() {
                memlog!("timeout while waiting for card to be ready");
                memlog_flush_and_reset!();
            }

            // let's not spam the card
            time::wait(Duration::from_millis(1));
        };

        memlog!("card ready @ {:?} (OCR={:#010x})", time::uptime(), ocr);

        Ok(())
    }

    /// Puts all the cards on the bus in the idle state
    fn reset_cards(&self) -> Result<(), Error> {
        self.send_command(Command::GoIdleState)?;
        memlog!("sent all cards to idle state @ {:?}", time::uptime());
        Ok(())
    }

    fn change_bus_width(&mut self, width: Width, switch: bool) -> Result<(), Error> {
        if switch {
            let data = match width {
                Width::B1 => 0x03B7_0000,
                // Width::B4 => 0x03B7_0100,
                Width::B8 => 0x03B7_0200,
            };
            self.send_command(Command::Switch { data })?;
        }

        const DTW_OFFSET: u8 = 1;
        const DTW_MASK: u32 = 0b11 << DTW_OFFSET;
        self.usdhc
            .PROT_CTRL
            .rmw(|r| (r & !DTW_MASK) | (u32::from(width.dtw()) << DTW_OFFSET));
        self.width = width;

        Ok(())
    }

    fn change_frequency(&self, f: Frequency) {
        memlog!("change_frequency({:?}) @ {:?}", f, time::uptime());

        // wait for the clock to stabilize
        while self.usdhc.PRES_STATE.read() & SDSTB == 0 {
            crate::memlog_try_flush();
        }

        // gate the SD clock before changing it
        self.usdhc
            .VEND_SPEC
            .rmw(|vend_spec| vend_spec & !VEND_SPEC_CKEN);

        let (sdclkfs, dvs) = match f {
            // 200 MHz / 64 / 9 = 347.222 KHz
            Frequency::K400 => (0x20, 8),
            // 200 MHz / 2 / 5 = 20 MHz
            Frequency::M20 => (0x01, 4),
        };

        self.usdhc.SYS_CTRL.rmw(|mut r| {
            const DVS_OFFSET: u8 = 4;
            const DVS_MASK: u32 = 0b1111 << DVS_OFFSET;
            const SDCLKFS_OFFSET: u8 = 8;
            const SDCLKFS_MASK: u32 = 0xff << SDCLKFS_OFFSET;

            // set divisor
            r &= !DVS_MASK;
            r |= dvs << DVS_OFFSET;

            // set prescaler
            r &= !SDCLKFS_MASK;
            r |= sdclkfs << SDCLKFS_OFFSET;

            r
        });

        const SDSTB: u32 = 1 << 3;

        // wait for the clock to stabilize
        while self.usdhc.PRES_STATE.read() & SDSTB == 0 {
            crate::memlog_try_flush();
        }

        // ungate the SD clock
        self.usdhc
            .VEND_SPEC
            .rmw(|vend_spec| vend_spec | VEND_SPEC_CKEN);
    }

    fn software_reset(&mut self) {
        memlog!("software reset START @ {:?}", time::uptime());

        // wait for CMD and DATA lines to become free
        let busy = PRES_STATE_CDIHB | PRES_STATE_CIHB;
        while self.usdhc.PRES_STATE.read() & busy != 0 {
            crate::memlog_try_flush();
        }

        // gate off the SD clock
        self.usdhc
            .VEND_SPEC
            .rmw(|vend_spec| vend_spec & !VEND_SPEC_CKEN);

        // reset the uSDHC peripheral
        const RSTA: u32 = 1 << 24;
        self.usdhc.SYS_CTRL.rmw(|r| r | RSTA);

        // wait for reset
        while self.usdhc.SYS_CTRL.read() & RSTA != 0 {
            crate::memlog_try_flush();
        }

        // no error because it doesn't communicate with the card
        let _ = self.change_bus_width(Width::B1, false);
        self.change_frequency(Frequency::K400);

        // recommended in the eMMC spec
        time::wait(Duration::from_millis(1));

        // send 80 clock cycles to the card
        const INITA: u32 = 1 << 27;
        self.usdhc.SYS_CTRL.rmw(|r| r | INITA);

        memlog!("software reset DONE @ {:?}", time::uptime());
    }

    fn get_card_status(&self, rca: Rca) -> Result<card::Status, Error> {
        let rsp = self.send_command(Command::SendStatus { rca })?;
        let status = card::Status::from(rsp);
        Ok(status?)
    }

    // low-level API
    /// When a command time outs the command inhibition bit needs to be cleared
    /// before attempting to send a new command
    #[allow(dead_code)]
    fn clear_command_inhibit(&self) {
        const SYS_CTRL_RSTC: u32 = 1 << 25;

        // clear the Command Inhibit bit
        self.usdhc.SYS_CTRL.rmw(|r| r | SYS_CTRL_RSTC);
        if util::wait_for_or_timeout(
            || self.usdhc.SYS_CTRL.read() & SYS_CTRL_RSTC == 0,
            default_timeout(),
        )
        .is_err()
        {
            memlog!("RSTC timeout");
            memlog_flush_and_reset!();
        }
    }

    /// [Blocking] send a command to the card
    fn send_command(&self, cmd: Command) -> Result<u32, Error> {
        if self.verbose {
            memlog!("send_command(cmd={:?}) @ {:?}", cmd, time::uptime());
        }
        debug_assert!(
            self.usdhc.INT_STATUS.read() & INT_STATUS_CC == 0,
            "INT_STATUS.CC bit was not cleared"
        );

        // wait until the bus is idle
        let busy = PRES_STATE_CIHB | PRES_STATE_CDIHB | PRES_STATE_DLA;
        while self.usdhc.PRES_STATE.read() & busy != 0 {
            crate::memlog_try_flush();
        }

        const RSTYP_OFFSET: u8 = 16;
        const CICEN: u32 = 1 << 20;
        const CCCEN: u32 = 1 << 19;
        const DPSEL: u32 = 1 << 21;
        const CMDTYP_OFFSET: u8 = 22;
        const CMDINX_OFFSET: u8 = 24;

        let mut w_cmd = u32::from(cmd.index()) << CMDINX_OFFSET;
        w_cmd |= u32::from(cmd.cmdtyp()) << CMDTYP_OFFSET;
        if cmd.data_present() {
            w_cmd |= DPSEL
        }
        if cmd.cicen() {
            w_cmd |= CICEN;
        }
        if cmd.cccen() {
            w_cmd |= CCCEN;
        }
        w_cmd |= u32::from(cmd.response_type()) << RSTYP_OFFSET;

        let cmd_arg = cmd.arg();

        // the timing between commands must be at least 8 SD clock cycles according to the spec
        // @ 20 MHz that's 400 ns; @ 400 KHz that's 20 us
        time::wait(Duration::from_micros(20));

        // command argument
        self.usdhc.CMD_ARG.write(cmd_arg);
        // issue the command
        self.usdhc.CMD_XFR_TYP.write(w_cmd);

        // then wait for a response
        let any_error = INT_STATUS_ANY_ERROR;
        let mut int_status = 0;
        let has_command_completed = || {
            int_status = self.usdhc.INT_STATUS.read();
            int_status & (any_error | INT_STATUS_CC) != 0
        };
        if util::wait_for_or_timeout(has_command_completed, default_timeout()).is_err() {
            return Err(Error::Timeout);
        }

        if int_status & INT_STATUS_CTOE != 0 {
            self.usdhc.INT_STATUS.clear(INT_STATUS_CTOE);
            Err(Error::Timeout)
        } else if int_status & any_error != 0 {
            self.usdhc.INT_STATUS.clear(int_status & any_error);
            Err(Error::Other)
        } else {
            self.usdhc.INT_STATUS.clear(INT_STATUS_CC);

            let rsp = self.usdhc.CMD_RSP0.read();
            let rspty = cmd.response();

            if rspty == cmd::Response::R1 || rspty == cmd::Response::R1b {
                let status = card::Status::from(rsp)?;
                if self.verbose {
                    memlog!("{:?}", status);
                }
            }

            // let the data transfer complete (if any)
            if cmd.data_present() {
                let any_error = INT_STATUS_ANY_ERROR;
                let transfer_done = INT_STATUS_TC | INT_STATUS_DINT;
                let mut int_status = 0;
                let has_command_completed = || {
                    int_status = self.usdhc.INT_STATUS.read();
                    int_status & any_error != 0 || int_status & transfer_done == transfer_done
                };
                if util::wait_for_or_timeout(has_command_completed, default_timeout())
                    .is_err()
                {
                    return Err(Error::Timeout);
                }

                if int_status & INT_STATUS_CTOE != 0 {
                    self.usdhc.INT_STATUS.clear(INT_STATUS_CTOE);
                    Err(Error::Timeout)
                } else if int_status & any_error != 0 {
                    self.usdhc.INT_STATUS.clear(int_status & any_error);
                    Err(Error::Other)
                } else {
                    self.usdhc.INT_STATUS.clear(transfer_done);
                    Ok(rsp)
                }
            } else {
                Ok(rsp)
            }
        }
    }
}

/// Command error
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Card did not respond in time.
    Timeout,
    /// Card is not in the transfer state (data cannot be accessed)
    NotInTransferState,
    /// Unclassified error.
    Other,
    /// Error condition in the card
    Card(card::Status),
}

impl From<card::Status> for Error {
    fn from(s: card::Status) -> Self {
        Error::Card(s)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Timeout => f.write_str("timeout"),
            Error::NotInTransferState => f.write_str("card not in transfer state"),
            Error::Other => f.write_str("unclassified error"),
            Error::Card(s) => write!(f, "card error (code: {:#010x})", s.bits),
        }
    }
}

impl ManagedBlockDevice for eMMC {
    type Error = Error;

    fn total_blocks(&self) -> u64 {
        u64::from(self.blocks)
    }

    fn read(&self, block: &mut Block, lba: u64) -> Result<(), Self::Error> {
        if lba > self.total_blocks() {
            return Err(Error::Other);
        }

        Self::read(self, lba as u32, block)?;
        Ok(())
    }

    fn write(&mut self, block: &Block, lba: u64) -> Result<(), Self::Error> {
        if lba > self.total_blocks() {
            return Err(Error::Other);
        }

        Self::write(self, lba as u32, block)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // no-operation
        Ok(())
    }
}
