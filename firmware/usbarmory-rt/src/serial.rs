use pac::{CCM, UART2};

pub fn init() {
    // configure UART2 pins
    unsafe {
        const SW_MUX_CTL_PAD_UART2_TX_DATA: *mut u32 = 0x20E_0094 as *mut u32;
        SW_MUX_CTL_PAD_UART2_TX_DATA.write_volatile(0);

        const SW_PAD_CTL_PAD_UART2_TX_DATA: *mut u32 = 0x20E_0320 as *mut u32;
        SW_PAD_CTL_PAD_UART2_TX_DATA.write_volatile(0x0001_b0b1);

        const SW_MUX_CTL_PAD_UART2_RX_DATA: *mut u32 = 0x20E_0098 as *mut u32;
        SW_MUX_CTL_PAD_UART2_RX_DATA.write_volatile(0);

        const SW_PAD_CTL_PAD_UART2_RX_DATA: *mut u32 = 0x20E_0324 as *mut u32;
        SW_PAD_CTL_PAD_UART2_RX_DATA.write_volatile(0x0001_b0b1);

        const UART2_RX_DATA_SELECT_INPUT: *mut u32 = 0x20E_062C as *mut u32;
        UART2_RX_DATA_SELECT_INPUT.write_volatile(0b01);
    }

    // ungate UART2's clock
    CCM::borrow_unchecked(|ccm| {
        ccm.CCGR0.rmw(|ccgr| ccgr | (0b11 << 28));
    });

    UART2::borrow_unchecked(|uart| {
        while uart.UTS.read() & 1 << 6 == 0 {
            continue;
        }

        // disable UART during configuration
        uart.UCR1.write(0);

        // software reset the UART
        uart.UCR2.write(0);
        while uart.UCR2.read() & 1 == 0 {
            continue;
        }

        // enable TX/RX, 8-bit data, 1-bit stop, no parity check, no RTS/CTS
        uart.UCR2.write(0x4027);
        // muxed mode (must be set); old auto-baud rate method
        uart.UCR3.write(0x0784);
        // reset value; not required?
        uart.UCR4.write(0x8000);

        // reference clock is input divided by 2 (40 MHz); DCE mode
        // uart.UFCR.write(0x0a01);

        // reference clock is input divided by 1 (80 MHz); DCE mode
        uart.UFCR.write(0x0a81);

        // configure baud rate: 3,000,000 baud (TODO: use 4Mbaud on Linux hosts)
        uart.UBIR.write(2);
        uart.UBMR.write(4);

        uart.UMCR.write(0);

        // UART mode / enable
        uart.UCR1.write(1);

        // // wait until transmission is done
        // // TODO remove?
        // while uart.UTS.read() & 1 << 6 == 0 {
        //     continue;
        // }

        // // disable UART during configuration
        // // TODO remove?
        // uart.UCR1.write(0);

        // // software reset the UART
        // // TODO remove?
        // uart.UCR2.write(0);
        // while uart.UCR2.read() & 1 == 0 {
        //     continue;
        // }

        // // enable TX/RX, 8-bit data, 1-bit stop, no parity check, no RTS/CTS
        // uart.UCR2.write(0x4027);
        // // muxed mode (must be set); old auto-baud rate method
        // uart.UCR3.write(0x0784);
        // // reset value; not required?
        // uart.UCR4.write(0x8000);

        // // reference clock is input divided by 1 (80 MHz); DCE mode
        // uart.UFCR.write(0x0a81);

        // // set baud rate to 80MHz / 20 = 4 MHz
        // uart.UBIR.write(0xf);
        // uart.UBMR.write(20 - 1);

        // // TODO remove?
        // uart.UMCR.write(0);

        // // TODO remove?
        // // UART mode / enable
        // uart.UCR1.write(1);
    });
}
