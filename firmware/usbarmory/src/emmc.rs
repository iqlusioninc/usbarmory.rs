//! Low level access to the eMMC

use core::{
    num::NonZeroU16,
    sync::atomic::{self, Ordering},
};

use pac::uSDHC2;

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
    ddr: bool,
}

type Rca = NonZeroU16;

/// Command complete
const INT_STATUS_CC: u32 = 1; // bit 0
/// Transfer complete
const INT_STATUS_TC: u32 = 1 << 1; // bit 1
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

/// Bits that must always be ones
const MIX_CTRL_RESERVED: u32 = 1 << 31;
/// Single block
const MIX_CTRL_MBSEL_SINGLE: u32 = 0 << 5;
/// Enable the DMA
const MIX_CTRL_DMAEN_ENABLE: u32 = 1; // bit 0
const MIX_CTRL_DDR_EN: u32 = 1 << 3;

/// Relative address assigned to the eMMC
// to make sure we are not making assumptions let's use a value other than the
// default one (0x01)
const RCA: Rca = unsafe { Rca::new_unchecked(0x02) };

#[derive(Debug)]
enum Frequency {
    K400,
    M48,
}

impl eMMC {
    /// Gets a handle to the `eMMC` singleton
    ///
    /// This method returns the `Some` only once. When successful this method
    /// consumes the `uSDHC2` peripheral
    pub fn take() -> Option<Result<Self, Error>> {
        uSDHC2::take().map(|usdhc| {
            let mut emmc = Self {
                usdhc,
                blocks: 0,
                selected: None,
                verbose: false,
                ddr: false,
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
            memlog!("read(block_nr={}) @ {:?}", block_nr, time::uptime());
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

    /// Waits until any previously written blocks have been programmed to flash
    pub fn flush(&self) -> Result<(), Error> {
        if let Some(rca) = self.selected {
            let status = self.get_card_status(rca, false)?;

            if status.state == card::State::Programming {
                // wait until the card finishes programming the data it received
                let start = Instant::now();
                while self.get_card_status(rca, false)?.state != card::State::Transfer {
                    crate::memlog_try_flush();

                    if Instant::now() - start > default_timeout() {
                        memlog!("card took too long to program to flash the data it received");
                        memlog_flush_and_reset!();
                    }
                }

                memlog!("card flushed @ {:?}", time::uptime());
            }
        }

        Ok(())
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

        let status = self.get_card_status(rca, false)?;
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
                | if self.ddr { MIX_CTRL_DDR_EN } else { 0 }
                | MIX_CTRL_MBSEL_SINGLE
                | MIX_CTRL_DTDSEL_READ
                | MIX_CTRL_DMAEN_ENABLE,
        );

        // semantically, the next register write transfers ownership of the
        // buffer to the DMA so ensure all previous memory operations complete
        // before the ownership transfer (NOTE this could be done a bit later:
        // in the middle of `send_command(WriteSingleBlock)`)
        atomic::fence(Ordering::Release);
        self.usdhc.DS_ADDR.write(addr as usize as u32);

        // start the transfer
        let cmd = block_nr
            .map(|block_nr| Command::ReadSingleBlock { block_nr })
            .unwrap_or(Command::SendExtCsd);
        self.send_command(cmd, false);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        let status = card::Status::from(self.usdhc.CMD_RSP0.read())?;
        if self.verbose {
            memlog!("started read ({:?})", status);
        }

        // wait for the transfer to finish
        // FIXME this could be non-blocking
        let mut int_status = 0;
        let has_transfer_completed = || {
            int_status = self.usdhc.INT_STATUS.read();
            int_status & (INT_STATUS_ANY_ERROR | INT_STATUS_TC) != 0
        };
        if util::wait_for_or_timeout(has_transfer_completed, default_timeout()).is_err() {
            memlog!("read: INT_STATUS.TC timeout");
            memlog_flush_and_reset!();
        }
        self.usdhc.INT_STATUS.clear(INT_STATUS_TC);

        // TODO accessing `buf` requires cache invalidation
        // let the DMA finish its memory operations before `block` is read
        atomic::fence(Ordering::Acquire);

        if int_status & INT_STATUS_ANY_ERROR != 0 {
            memlog!("read: data error (INT_STATUS={:#010x})", int_status);
            memlog_flush_and_reset!();
        }

        if self.verbose {
            memlog!("read DONE @ {:?}", time::uptime());
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

        let status = self.get_card_status(rca, false)?;
        if self.verbose {
            memlog!("{:?}", status);
        }

        if !status.ready_for_data && status.state == card::State::Transfer {
            return Err(Error::NotInTransferState);
        }

        /// Data Transfer Direction Select = Write (Host to Card)
        const MIX_CTRL_DTDSEL_WRITE: u32 = 0 << 4;
        self.usdhc.MIX_CTRL.write(
            MIX_CTRL_RESERVED
                | if self.ddr { MIX_CTRL_DDR_EN } else { 0 }
                | MIX_CTRL_MBSEL_SINGLE
                | MIX_CTRL_DTDSEL_WRITE
                | MIX_CTRL_DMAEN_ENABLE,
        );

        // semantically, the next register write transfers ownership of the
        // buffer to the DMA so ensure all previous memory operations complete
        // before the ownership transfer (NOTE this could be done a bit later:
        // in the middle of `send_command(WriteSingleBlock)`)
        atomic::fence(Ordering::Release);
        self.usdhc.DS_ADDR.write(addr as usize as u32);

        // start the transfer
        self.send_command(Command::WriteSingleBlock { block_nr }, false);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        let status = card::Status::from(self.usdhc.CMD_RSP0.read())?;
        if self.verbose {
            memlog!("{:?}", status);
        }

        // wait for the transfer to finish
        // FIXME this could be non-blocking
        let mut int_status = 0;
        let has_transfer_completed = || {
            int_status = self.usdhc.INT_STATUS.read();
            int_status & (INT_STATUS_ANY_ERROR | INT_STATUS_TC) != 0
        };
        if util::wait_for_or_timeout(has_transfer_completed, default_timeout()).is_err() {
            memlog!("write: INT_STATUS.TC timeout");
            memlog_flush_and_reset!();
        }
        self.usdhc.INT_STATUS.clear(INT_STATUS_TC);

        // buffer handled back to us
        atomic::fence(Ordering::Acquire);

        if int_status & INT_STATUS_ANY_ERROR != 0 {
            memlog!("write: data error (INT_STATUS={:#010x})", int_status);
            memlog_flush_and_reset!();
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

        // set block size (host side)
        const BLKSIZE_OFFSET: u8 = 0;
        const BLKCNT_OFFSET: u8 = 16;
        assert!(BLOCK_SIZE < (1 << 13));
        // XXX this needs to change in the case of multi-block transfers
        let blkcnt = 1;
        self.usdhc
            .BLK_ATT
            .write(blkcnt << BLKCNT_OFFSET | u32::from(BLOCK_SIZE) << BLKSIZE_OFFSET);

        // TODO configure the host and the card to do 4-bit DDR transfers at 26,
        // or 52, MHz
        // see sections 56.5.4.3 & 56.5.4.4

        // 1-bit data mode (host side)
        const PROT_CTRL_DTW_MASK: u32 = 0b11 << 1;
        // 8-bit mode
        // const PROT_CTRL_DTW8: u32 = 0b10 << 1;
        // 4-bit mode
        // const PROT_CTRL_DTW4: u32 = 0b01 << 1;
        /// 1-bit mode
        const PROT_CTRL_DTW1: u32 = 0b00 << 1;
        const PROT_CTRL_DMASEL_MASK: u32 = 0b11 << 8;
        /// Simple DMA mode
        const PROT_CTRL_DMASEL_SIMPLE: u32 = 0b00 << 8;

        self.usdhc.PROT_CTRL.rmw(|mut r| {
            r &= !PROT_CTRL_DTW_MASK;
            r |= PROT_CTRL_DTW1;

            r &= !PROT_CTRL_DMASEL_MASK;
            r |= PROT_CTRL_DMASEL_SIMPLE;

            r
        });

        // select the card
        self.send_command(Command::SelectCard { rca: Some(rca) }, true);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        card::Status::from(self.usdhc.CMD_RSP0.read())?;

        // set block size (card side)
        self.send_command(
            Command::SetBlockLen {
                len: BLOCK_SIZE.into(),
            },
            true,
        );
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        card::Status::from(self.usdhc.CMD_RSP0.read())?;

        self.selected = Some(rca);

        Ok(())
    }

    /// Returns the card specific data of the card with the specified relative
    /// address
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
        // assign a different relative address to each one
        memlog!("registering cards on the bus");
        self.send_command(Command::AllSendCid, true);

        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        let _cid = [
            self.usdhc.CMD_RSP0.read(),
            self.usdhc.CMD_RSP1.read(),
            self.usdhc.CMD_RSP2.read(),
            self.usdhc.CMD_RSP3.read(),
        ];

        self.send_command(Command::SetRelativeAddr { rca: RCA }, true);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        let status = self.usdhc.CMD_RSP0.read();
        memlog!(
            "registered a card with RCA={:#04x} ({:?})",
            RCA,
            card::Status::from(status)?
        );

        // this should fail with a timeout because there no more cards on the
        // bus
        self.send_command(Command::AllSendCid, true);
        let res = self.wait_response();

        if res != Err(Error::Timeout) {
            memlog!("expected command timeout but got {:?}", res);
            memlog_flush_and_reset!();
        }

        self.clear_command_inhibit();

        memlog!("no more cards on the bus");

        Ok(())
    }

    /// Selects an operating voltage that supports most (all?) cards on the bus
    fn voltage_validation(&self) {
        // query supported voltage range
        self.send_command(Command::SendOpCond { voltage_range: 0 }, true);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }

        let mut ocr = self.usdhc.CMD_RSP0.read();
        memlog!(
            "queried supported voltage range (OCR={:#010x}) @ {:?}",
            ocr,
            time::uptime()
        );

        /// 0 = Card is busy; 1 = Card has finished powering up
        const OCR_RDY: u32 = 1 << 31;

        // broadcast operating voltage; wait until cards have finished powering up
        let start = Instant::now();
        let voltage_range = ocr & !OCR_RDY;
        while ocr & OCR_RDY == 0 {
            self.send_command(Command::SendOpCond { voltage_range }, false);
            if let Err(e) = self.wait_response() {
                memlog!("command response error: {:?}", e);
                memlog_flush_and_reset!();
            }
            ocr = self.usdhc.CMD_RSP0.read();

            if start.elapsed() > default_timeout() {
                memlog!("timeout while waiting for card to be ready");
                memlog_flush_and_reset!();
            }
        }

        memlog!("Card ready @ {:?}", time::uptime());
    }

    /// Puts all the cards on the bus in the idle state
    fn reset_cards(&self) {
        self.send_command(Command::GoIdleState, true);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        memlog!("sent all cards to idle state @ {:?}", time::uptime());
    }

    fn change_frequency(&self, f: Frequency) {
        memlog!("change_frequency({:?}) @ {:?}", f, time::uptime());
        const FRC_SDCLK_ON: u32 = 1 << 8;

        // clear FRC_SDCLK_ON before changing the clock
        self.usdhc
            .VEND_SPEC
            .rmw(|vend_spec| vend_spec & !FRC_SDCLK_ON);

        // wait for the clock to stabilize before changing the frequency
        const SDSTB: u32 = 1 << 3;
        while self.usdhc.PRES_STATE.read() & SDSTB == 0 {
            crate::memlog_try_flush();
        }

        let (dvs, sdclkfs) = match f {
            Frequency::K400 => (15, 0x10),
            Frequency::M48 => (0, 2),
        };

        // reduce clock speed to 400 KHz; after reset the card will operate at its lowest speed
        self.usdhc.SYS_CTRL.rmw(|mut r| {
            const DTOCV_OFFSET: u8 = 16;
            const DTOCV_MASK: u32 = 0xf << DTOCV_OFFSET;
            const DVS_OFFSET: u8 = 4;
            const DVS_MASK: u32 = 0b1111 << DVS_OFFSET;
            const SDCLKFS_OFFSET: u8 = 8;
            const SDCLKFS_MASK: u32 = 0xff << SDCLKFS_OFFSET;

            // set data timeout counter to 1<<27
            r &= !DTOCV_MASK;
            r |= 14 << DTOCV_OFFSET;

            // set divisor
            r &= !DVS_MASK;
            r |= dvs << DVS_OFFSET;

            // set prescaler
            r &= !SDCLKFS_MASK;
            r |= sdclkfs << SDCLKFS_OFFSET;

            // lowest 4 bits must always be ones
            r |= 0xf;

            // clock frequency = 192 MHz / divisor / prescaler
            r
        });

        // wait for the clock to stabilize
        while self.usdhc.PRES_STATE.read() & SDSTB == 0 {
            crate::memlog_try_flush();
        }
    }

    fn software_reset(&self) {
        memlog!("start uSDHC2 reset @ {:?}", time::uptime());

        // clear FRC_SDCLK_ON before setting RSTA
        const FRC_SDCLK_ON: u32 = 1 << 8;
        self.usdhc
            .VEND_SPEC
            .rmw(|vend_spec| vend_spec & !FRC_SDCLK_ON);

        // reset the uSDHC peripheral
        const RSTA: u32 = 1 << 24;
        const RESERVED: u32 = 0xf; // write as 1 reserved bits
        self.usdhc.SYS_CTRL.rmw(|r| r | RSTA | RESERVED);

        // wait for reset
        while self.usdhc.SYS_CTRL.read() & RSTA != 0 {
            crate::memlog_try_flush();
        }

        // reset registers that RSTA doesn't touch
        self.usdhc.MMC_BOOT.reset();
        self.usdhc.MIX_CTRL.reset();
        self.usdhc.CLK_TUNE_CTRL_STATUS.reset();
        self.usdhc.VEND_SPEC.reset();
        self.usdhc.DLL_CTRL.write(0);

        // select 1-bit mode; we'll reset the card so it will be operating in 1-bit mode
        self.usdhc.PROT_CTRL.reset();

        self.change_frequency(Frequency::K400);

        // wait for CMD and DATA lines to become free
        let busy = PRES_STATE_CDIHB | PRES_STATE_CIHB;
        while self.usdhc.PRES_STATE.read() & busy != 0 {
            crate::memlog_try_flush();
        }

        // send 80 clock cycles to the card
        const INITA: u32 = 1 << 27;
        self.usdhc.SYS_CTRL.rmw(|r| r | INITA);

        memlog!("reset DONE @ {:?}", time::uptime());
    }

    // TODO this should bubble up errors
    fn get_card_status(&self, rca: Rca, verbose: bool) -> Result<card::Status, Error> {
        self.send_command(Command::SendStatus { rca }, verbose);
        if let Err(e) = self.wait_response() {
            memlog!("command response error: {:?}", e);
            memlog_flush_and_reset!();
        }
        let status = card::Status::from(self.usdhc.CMD_RSP0.read());
        if verbose {
            memlog!("status={:?}", status);
        }
        Ok(status?)
    }

    // low-level API
    /// When a command time outs the command inhibition bit needs to be cleared
    /// before attempting to send a new command
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

    /// Send a command
    fn send_command(&self, cmd: Command, verbose: bool) {
        if verbose {
            memlog!("send_command(cmd={:?}) @ {:?}", cmd, time::uptime());
        }
        assert!(
            self.usdhc.INT_STATUS.read() & INT_STATUS_CC == 0,
            "previous command was not checked"
        );
        let pres_state = self.usdhc.PRES_STATE.read();
        assert!(pres_state & PRES_STATE_CIHB == 0, "CMD line is being used");
        if cmd.uses_data_line() {
            assert!(
                pres_state & PRES_STATE_CDIHB == 0,
                "DATA line is being used"
            );
        }

        const CICEN: u32 = 1 << 20;
        const CCCEN: u32 = 1 << 19;
        const RSTYP_OFFSET: u8 = 16;
        const DPSEL_OFFSET: u8 = 21;
        const CMDTYP_OFFSET: u8 = 22;
        const CMDINX_OFFSET: u8 = 24;

        let mut w_cmd = u32::from(cmd.index()) << CMDINX_OFFSET;
        w_cmd |= u32::from(cmd.cmdtyp()) << CMDTYP_OFFSET;
        w_cmd |= if cmd.data_present() {
            1 << DPSEL_OFFSET
        } else {
            0
        };
        if cmd.cicen() {
            w_cmd |= CICEN;
        }
        if cmd.cccen() {
            w_cmd |= CCCEN;
        }
        w_cmd |= u32::from(cmd.response_type()) << RSTYP_OFFSET;

        let cmd_arg = cmd.arg();

        // command argument
        self.usdhc.CMD_ARG.write(cmd_arg);
        // issue the command
        self.usdhc.CMD_XFR_TYP.write(w_cmd);
    }

    /// [blocking] Waits for a command response
    fn wait_response(&self) -> Result<(), Error> {
        let any_error = INT_STATUS_CTOE | INT_STATUS_CCE | INT_STATUS_CEBE | INT_STATUS_CIE;
        let mut int_status = 0;
        let has_command_completed = || {
            int_status = self.usdhc.INT_STATUS.read();
            int_status & (any_error | INT_STATUS_CC) != 0
        };
        if util::wait_for_or_timeout(has_command_completed, default_timeout()).is_err() {
            memlog!("INT_STATUS.CC timeout");
            memlog_flush_and_reset!();
        }

        self.usdhc.INT_STATUS.clear(INT_STATUS_CC);

        if int_status & INT_STATUS_CTOE != 0 {
            Err(Error::Timeout)
        } else if int_status & any_error != 0 {
            Err(Error::Other)
        } else {
            Ok(())
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
    /// Unknown error.
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
            Error::Other => f.write_str("unknown error"),
            Error::Card(s) => write!(f, "card error (code: {})", s.bits),
        }
    }
}

impl ManagedBlockDevice for eMMC {
    type Error = Error;

    fn total_blocks(&self) -> u64 {
        self.blocks.into()
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
        Self::flush(self)?;
        Ok(())
    }
}
