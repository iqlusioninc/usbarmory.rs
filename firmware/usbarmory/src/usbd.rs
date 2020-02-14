//! USB - device mode

#![allow(dead_code)]

mod bus;
mod dqh;
mod dtd;
mod token;
mod util;

use heapless::Vec;
use pac::{ccm_analog::CCM_ANALOG, usb_analog::USB_ANALOG, usb_uog::USB_UOG1, usbphy::USBPHY1};
use spin::Mutex;
use typenum::marker_traits::Unsigned;

use crate::{memlog, memlog_flush_and_reset};
use dqh::dQH;
use dtd::dTD;
use util::Align2K;

/// USB device
pub struct Usbd {
    // NOTE for now the Mutex is used just to satisfy the undocumented `Sync`
    // bound required by the `UsbBus` trait
    inner: Mutex<Inner>,
}

// Number of supported endpoints: 2 IN & 2 OUT. 1 IN-OUT pair is used for the
// control endpoint 0. Currently this is hardcoded by could be made configurable
const ENDPOINTS: usize = 4;

// Maximum number of dTD that can be used
type NDTDS = heapless::consts::U4;
// Numbers of buffers managed by `Usbd`
type NBUFS = heapless::consts::U1;

impl Usbd {
    /// Gets a handle to the USB device
    ///
    /// This returns the `Some` variant only once. This method consumes the
    /// CCM_ANALOG, USBPHY1, USB_ANALOG and USB_UOG1 peripherals.
    pub fn take() -> Option<Self> {
        if let (Some(ccm_analog), Some(usbphy), Some(usb_analog), Some(usb)) = (
            CCM_ANALOG::take(),
            USBPHY1::take(),
            USB_ANALOG::take(),
            USB_UOG1::take(),
        ) {
            // # initialize some data structures

            // NOTE this code runs in a critical section and runs only once
            static mut DQHS: Align2K<[dQH; ENDPOINTS]> = Align2K {
                inner: [dQH::new(), dQH::new(), dQH::new(), dQH::new()],
            };

            static mut DTDS: [dTD; NDTDS::USIZE] = [dTD::new(), dTD::new(), dTD::new(), dTD::new()];

            let mut dtds = Vec::new();
            unsafe {
                for dtd in DTDS.iter_mut() {
                    dtds.push(dtd).ok().expect("UNREACHABLE");
                }
            }

            static mut B512S: [[u8; 512]; NBUFS::USIZE] = [[0; 512]; 1];

            let mut b512s = Vec::new();
            unsafe {
                for b512 in B512S.iter_mut() {
                    b512s.push(b512).ok().expect("UNREACHABLE");
                }
            }

            // NOTE(unsafe) this code runs exactly once; this is an owning
            // pointer (it won't be aliased)
            let endptlistaddr = unsafe { DQHS.inner.as_ptr() };

            // # Configure the USB clock
            // NOTE based on tamago's [1] USB code: USBx.Init @ imx6/usb/bus.go
            // [1]: https://github.com/f-secure-foundry/tamago @ 4195e27d20950715dbf11c3b9dbf77a5a4431910

            /// Powers up the PLL
            const CCM_ANALOG_PLL_USB1_POWER: u32 = 1 << 12;

            ccm_analog.PLL_USB1_SET.write(CCM_ANALOG_PLL_USB1_POWER);

            /// 1 = PLL outputs for USBPHY1 on
            const CCM_ANALOG_PLL_USB1_EN_USB_CLKS: u32 = 1 << 6;

            ccm_analog
                .PLL_USB1_SET
                .write(CCM_ANALOG_PLL_USB1_EN_USB_CLKS);

            /// 1 = PLL is currently locked
            const CCM_ANALOG_PLL_USB1_LOCK: u32 = 1 << 31;

            // wait for the PLL to lock
            loop {
                if ccm_analog.PLL_USB1.read() & CCM_ANALOG_PLL_USB1_LOCK != 0 {
                    // PLL is locked
                    break;
                }

                crate::memlog_try_flush();
            }

            /// Bypass the PLL
            const CCM_ANALOG_PLL_USB1_BYPASS: u32 = 1 << 31;

            ccm_analog.PLL_USB1_CLR.write(CCM_ANALOG_PLL_USB1_BYPASS);

            /// Enable the PLL clock output
            const CCM_ANALOG_PLL_USB1_ENABLE: u32 = 1 << 13;

            ccm_analog.PLL_USB1_SET.write(CCM_ANALOG_PLL_USB1_ENABLE);

            // Seal the CCM configuration
            drop(ccm_analog);

            /// 1 = held USBPHY in reset
            const USBPHY_CTRL_SFTRST: u32 = 1 << 31;

            usbphy.CTRL_SET.write(USBPHY_CTRL_SFTRST);
            usbphy.CTRL_CLR.write(USBPHY_CTRL_SFTRST);

            /// 0 = run the UTMI clocks
            const USBPHY_CTRL_CLKGATE: u32 = 1 << 30;

            usbphy.CTRL_CLR.write(USBPHY_CTRL_CLKGATE);

            /// 1 = Power-down the entire USB PHY receiver block
            const USBPHY_PWD_RXPWDRX: u32 = 1 << 20;
            /// 1 = Power-down the USB high-speed differential receiver
            const USBPHY_PWD_RXPWDDIFF: u32 = 1 << 19;
            /// 1 = Power-down the USB full-speed differential receiver
            const USBPHY_PWD_RXPWD1PT1: u32 = 1 << 18;
            /// 1 = Power-down the USB high-speed receiver envelope detector
            const USBPHY_PWD_RXPWDENV: u32 = 1 << 17;
            /// 1 = Power-down the USB PHY transmit V-to-I converter and the
            /// current minor
            const USBPHY_PWD_TXPWDV2I: u32 = 1 << 12;
            /// 1 = Power-down the USB PHY current bias block for the transmitter
            const USBPHY_PWD_TXPWDIBIAS: u32 = 1 << 11;
            /// 1 = Power-down the USB full-speed drivers
            const USBPHY_PWD_TXPWDFS: u32 = 1 << 10;

            // power *up* all the USB things
            usbphy.PWD_CLR.write(
                USBPHY_PWD_RXPWDRX
                    | USBPHY_PWD_RXPWDDIFF
                    | USBPHY_PWD_RXPWD1PT1
                    | USBPHY_PWD_RXPWDENV
                    | USBPHY_PWD_TXPWDV2I
                    | USBPHY_PWD_TXPWDIBIAS
                    | USBPHY_PWD_TXPWDFS,
            );

            /// Enables UTMI+ Level 2. This should be enabled if needs to support LS
            /// device
            const USBPHY_CTRL_ENUTMILEVEL2: u32 = 1 << 14;
            /// Enables UTMI+ Level 3. This should be enabled if needs to support
            /// external FS Hub with LS device connected
            const USBPHY_CTRL_ENUTMILEVEL3: u32 = 1 << 15;

            usbphy.CTRL_SET.write(USBPHY_CTRL_ENUTMILEVEL2);
            usbphy.CTRL_SET.write(USBPHY_CTRL_ENUTMILEVEL3);

            /// For host mode, enables high-speed disconnect detector
            const USBPHY_CTRL_ENHOSTDISCONDETECT: u32 = 1 << 1;

            usbphy.CTRL_SET.write(USBPHY_CTRL_ENHOSTDISCONDETECT);

            // seal the PHY configuration
            drop(usbphy);

            /// 1 = Check whether a charger is connected to the USB port
            const USB_ANALOG_USB1_CHARG_DETECT_EN_B: u32 = 1 << 20;
            /// 1 = Enable the charger detector
            const USB_ANALOG_USB1_CHARG_DETECT_CHK_CHRG_B: u32 = 1 << 19;

            // disable charger detector
            usb_analog
                .USB1_CHRG_DETECT_SET
                .write(USB_ANALOG_USB1_CHARG_DETECT_CHK_CHRG_B);
            usb_analog
                .USB1_CHRG_DETECT_SET
                .write(USB_ANALOG_USB1_CHARG_DETECT_EN_B);

            // seal the USB analog configuration
            drop(usb_analog);

            const USB_OTG_USBCMD_RST: u32 = 1 << 1;

            usb.USBCMD.rmw(|usbmode| usbmode | USB_OTG_USBCMD_RST);
            if util::wait(
                || usb.USBCMD.read() & USB_OTG_USBCMD_RST == 0,
                2 * consts::frame(),
            )
            .is_err()
            {
                memlog!("USB hardware reset timeout");
                memlog_flush_and_reset!();
            }

            /// Device-only controller
            const USB_OTG_USBMODE_CM_DEVICE: u32 = 0b10;
            const USB_OTG_USBMODE_CM_MASK: u32 = 0b11;
            /// 1 = Setup Lockouts Off
            const USB_OTG_USBMODE_SLOM: u32 = 1 << 3;
            const USB_OTG_USBMODE_SDIS: u32 = 1 << 4;

            usb.USBMODE
                .rmw(|usbmode| (usbmode & !USB_OTG_USBMODE_CM_MASK) | USB_OTG_USBMODE_CM_DEVICE);

            usb.USBMODE.rmw(|usbmode| usbmode | USB_OTG_USBMODE_SLOM);

            usb.USBMODE.rmw(|usbmode| usbmode & !USB_OTG_USBMODE_SDIS);

            if util::wait(
                || usb.USBMODE.read() & USB_OTG_USBMODE_CM_MASK == USB_OTG_USBMODE_CM_DEVICE,
                2 * consts::frame(),
            )
            .is_err()
            {
                memlog!("switching to USB device mode timeout");
                memlog_flush_and_reset!();
            }

            // set dQH list -- here we are effectively transferring ownership of
            // the owning pointer to the peripheral
            usb.ENDPTLISTADDR.write(endptlistaddr as usize as u32);

            /// OTG Termination. This bit must be set when the OTG device is in
            /// device mode
            const USB_OTG_OTGSC_OT: u32 = 1 << 3;

            usb.OTGSC.rmw(|otgsc| otgsc | USB_OTG_OTGSC_OT);

            Some(Self {
                inner: Mutex::new(Inner {
                    usb,
                    dtds,
                    b512s,
                    used_dqhs: 0,
                    setupstat: None,
                    ep_in_complete: None,
                    last_poll_was_none: false,
                    needs_status_out: false,
                }),
            })
        } else {
            None
        }
    }
}

struct Inner {
    usb: USB_UOG1,

    // memory management
    dtds: Vec<&'static mut dTD, NDTDS>,
    b512s: Vec<&'static mut [u8; 512], NBUFS>,

    // bitmask that indicates which endpoints are currently in use
    used_dqhs: u8, // NOTE must be updated if `ENDPOINTS` changes

    setupstat: Option<u16>,
    ep_in_complete: Option<u16>,
    last_poll_was_none: bool,
    needs_status_out: bool,
}
