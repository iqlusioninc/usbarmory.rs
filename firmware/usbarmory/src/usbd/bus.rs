//! `UsbBus` implementation

use core::sync::atomic::{self, Ordering};

use usb_device::{
    bus::{PollResult, UsbBus},
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection, UsbError,
};

use super::{
    dqh::dQH,
    token::{Status, Token},
    util::{self, Data, Hex, Ref},
    Inner, Usbd, ENDPOINTS,
};
use crate::{memlog, memlog_flush_and_reset, time};

impl UsbBus for Usbd {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        // NOTE we are using this in single-threaded context so deadlocks are
        // impossible
        self.inner.try_lock().expect("UNREACHABLE").alloc_ep(
            ep_dir,
            ep_addr,
            ep_type,
            max_packet_size,
            interval,
        )
    }

    fn enable(&mut self) {
        self.inner.try_lock().expect("UNREACHABLE").enable();
    }

    fn is_stalled(&self, _: EndpointAddress) -> bool {
        false
    }

    fn poll(&self) -> PollResult {
        // NOTE we are using this in single-threaded context so deadlocks are
        // impossible
        self.inner.try_lock().expect("UNREACHABLE").poll()
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        // NOTE we are using this in single-threaded context so deadlocks are
        // impossible
        self.inner
            .try_lock()
            .expect("UNREACHABLE")
            .read(ep_addr, buf)
    }

    fn reset(&self) {
        // NOTE we are using this in single-threaded context so deadlocks are
        // impossible
        self.inner.try_lock().expect("UNREACHABLE").reset();
    }

    fn resume(&self) {
        // TODO
    }

    fn set_stalled(&self, _: EndpointAddress, stalled: bool) {
        if stalled {
            // FIXME handle stall conditions
            unimplemented!()
        }
    }

    fn suspend(&self) {
        // TODO
    }

    fn set_device_address(&self, addr: u8) {
        memlog!("set_device_address({})", addr);
        crate::memlog_try_flush();

        // "instantaneous" udpate
        self.inner
            .try_lock()
            .expect("UNREACHABLE")
            .usb
            .DEVICEADDR
            .write((addr as u32) << 25);
    }

    fn write(&self, ep_addr: EndpointAddress, bytes: &[u8]) -> Result<usize, UsbError> {
        self.inner
            .try_lock()
            .expect("UNREACHABLE")
            .write(ep_addr, bytes)
    }
}

/// USB Reset Received
const USBSTS_URI: u32 = 1 << 6;

/// Port Change Detect
const USBSTS_PCI: u32 = 1 << 2;

impl Inner {
    // # UsbBus methods
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        memlog!(
            "alloc_ep(ep_dir={:?}, ep_addr={:?}, ep_type={:?}, max_packet_size={}, interval={}) @ {:?}",
            ep_dir,
            ep_addr,
            ep_type,
            max_packet_size,
            interval,
            time::uptime(),
        );

        let (ep_addr, dqh) = if let Some(ep_addr) = ep_addr {
            let dqh = self.get_dqh(ep_addr).ok_or(UsbError::EndpointOverflow)?;

            if self.is_ep_being_used(ep_addr) {
                return Err(UsbError::InvalidEndpoint);
            }

            (ep_addr, dqh)
        } else {
            // use the lowest endpoint address available that's not control 0
            let mut ep_idx = 1;
            loop {
                let ep_addr = EndpointAddress::from_parts(ep_idx, ep_dir);

                let dqh = self.get_dqh(ep_addr).ok_or(UsbError::EndpointOverflow)?;

                if !self.is_ep_being_used(ep_addr) {
                    break (ep_addr, dqh);
                } else {
                    // try next
                    ep_idx += 1;
                }
            }
        };

        // XXX You are supposed to write USB wMaxPacketSize in this field but
        // for unknown reasons if you write *exactly* that value then only the
        // *first* transfer (in either direction) through this endpoint will
        // work. Attempting to do a second transfer will result in the DMA
        // stalling with no error or status flag being raised. Writing a
        // slightly higher value than wMaxPacketSize appears to work around the
        // problem ...
        const MAX_PACKET_SIZE_HACK: u16 = 8;
        // NOTE(unsafe) dQH cannot access this field yet
        unsafe {
            dqh.set_max_packet_size(max_packet_size + MAX_PACKET_SIZE_HACK);
        }

        // TODO install a dTD for other endpoints
        if ep_addr.index() == 0 {
            unsafe {
                let dtd = Ref::new(self.dtds.pop().expect("exhausted the dTD pool"));

                // "Set the terminate bit to 1"
                dtd.set_next_dtd(None);

                // "Fill in total bytes with transfer size"
                // "Set the interrupt on complete if desired"
                // "Initialize the status field with the active bit set to 1 and
                // all remaining status bits set to 0"
                let mut token = Token::empty();
                if ep_addr.is_out() {
                    // XXX it may not make sense to do this here
                    token.set_total_bytes(usize::from(max_packet_size));
                    token.set_status(Status::active());
                }
                dtd.set_token(token);

                // "Fill in buffer pointer 0 and the current offset to point to
                // the start of the data buffer" -- this and the following steps
                // will be done in `read`

                dqh.set_next_dtd(Some(dtd));
            }
        }

        // NOTE no memory barrier here because we are not going to hand this to
        // the hardware just yet
        drop(dqh);

        // mark this endpoint as used
        self.mark_ep_as_used(ep_addr);

        Ok(ep_addr)
    }

    fn enable(&mut self) {
        /// Run/Stop. Writing a one to this bit will cause the controller to
        /// initialize an attach event
        const USBCMD_RS: u32 = 1;

        self.usb.USBCMD.rmw(|usbcmd| usbcmd | USBCMD_RS);
    }

    fn reset(&mut self) {
        // Handle a bus reset -- See section 54.4.6.2.1 of the ULRM

        // "Clear all setup token semaphores by reading the ENDPTSETUPSTAT
        // register and writing the same value back to the ENDPTSETUPSTAT
        // register"
        self.usb.ENDPTSETUPSTAT.rmw(|r| r);

        // "Clear all the endpoint complete status bits by reading the
        // ENDPTCOMPLETE" register and writing the same value back to the
        // ENDPTCOMPLETE register"
        self.usb.ENDPTCOMPLETE.rmw(|r| r);

        // "Cancel all primed status by waiting until all bits in the ENDPTPRIME
        // are 0 and then writing `!0` to ENDPTFLUSH"
        if util::wait(|| self.usb.ENDPTPRIME.read() == 0, 2 * consts::frame()).is_err() {
            memlog!("reset: ENDPTPRIME timeout");
            memlog_flush_and_reset!()
        }
        self.usb.ENDPTFLUSH.write(!0);

        /// Port Reset
        const PORTSC1_PR: u32 = 1 << 8;

        // "Read the reset bit in the PORTSC1 register and make sure that it is
        // still active"
        let portsc1 = self.usb.PORTSC1.read();
        if portsc1 & PORTSC1_PR == 0 {
            memlog!(
                "reset: we were too slow at handling the bus reset? (PORTSC1={:#010x})",
                portsc1
            );
        }

        // clear the URI bit
        self.usb.USBSTS.write(USBSTS_URI);

        memlog!("finished handling bus reset @ {:?}", time::uptime());
        crate::memlog_try_flush();
    }

    fn poll(&mut self) -> PollResult {
        /// When a controller enters a suspend state from an active state
        const USBSTS_SLI: u32 = 1 << 8;
        /// System error
        const USBSTS_SEI: u32 = 1 << 4;

        let sts = self.usb.USBSTS.read();
        let setupstat = self.usb.ENDPTSETUPSTAT.read() as u16;
        let complete = self.usb.ENDPTCOMPLETE.read();

        if complete != 0 {
            memlog!("ENDPTCOMPLETE: {:?}", &Hex(complete));
            memlog_flush_and_reset!();
        }

        if sts & USBSTS_PCI != 0 {
            self.port_change();
        }

        if setupstat != 0 {
            // cache `setuptstat`; it needs special handling in `read`
            self.setupstat = Some(setupstat);
        }

        if setupstat != 0 || self.ep_in_complete.is_some() || self.needs_status_out {
            let ep_setup = setupstat;
            let ep_in_complete = self.ep_in_complete.take().unwrap_or(0);
            // STATUS out needs to be reported after the IN data phase
            let ep_out = if self.needs_status_out && ep_in_complete == 0 {
                // TODO generalize to control endpoints other than 0
                self.needs_status_out = false;
                1
            } else {
                // the TX bits in complete shouldn't be reported
                assert!(complete < (1 << 16));

                complete as u16
            };

            let data = Data {
                ep_in_complete,
                ep_setup,
                ep_out,
            };

            memlog!("poll() -> {:?}", data);
            crate::memlog_try_flush();

            return data.into();
        }

        if sts & USBSTS_URI != 0 {
            self.last_poll_was_none = false;
            memlog!("poll() -> Reset @ {:?}", time::uptime());
            PollResult::Reset
        } else {
            if !self.last_poll_was_none {
                self.last_poll_was_none = true;
                memlog!("poll() -> None (USBSTS = {:#010x})", sts);
            }
            crate::memlog_try_flush();

            PollResult::None
        }
    }

    fn read(&mut self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        memlog!(
            "read(ep_addr={:?}, buf_len={}, self.setupstat={:?}) ... @ {:?}",
            ep_addr,
            buf.len(),
            self.setupstat,
            time::uptime()
        );
        crate::memlog_try_flush();

        assert_eq!(ep_addr.index(), 0, "not yet implemented");

        let setupstat = if let Some(setupstat) = self.setupstat.as_mut() {
            let mask = 1 << ep_addr.index();
            if *setupstat & mask != 0 {
                *setupstat &= !mask;
                Some(mask)
            } else {
                None
            }
        } else {
            None
        };

        let dqh = self.get_dqh(ep_addr).expect("UNREACHABLE");
        if let Some(setupstat) = setupstat {
            // SETUP packets need special handling because no dTD is used
            // see section 54.4.6.4.2.1 of the ULRM

            // 1. "Write 1 to clear corresponding bit in ENDPTSETUPSTAT"
            self.usb.ENDPTSETUPSTAT.write(u32::from(setupstat));

            const CMD_SUTW: u32 = 1 << 13;

            let n = dQH::SETUP_BYTES;
            loop {
                // 2. "Write 1 to Setup Tripwire (SUTW) in USBCMD"
                self.usb.USBCMD.rmw(|cmd| cmd | CMD_SUTW);

                // 3. "Duplicate contents of dQH.SetupBuffer into local software
                // byte array"
                dqh.copy_setup_bytes(&mut buf[..n]);

                // 4. "Read Setup TripWire (SUTW) in USBCMD. If set continue; if
                // cleared go to 2"
                if self.usb.USBCMD.read() & CMD_SUTW != 0 {
                    break;
                } else {
                    continue;
                }
            }

            drop(dqh);

            // 5. "Write 0 to clear Setup Tripwire (SUTW) in USBCMD"
            self.usb.USBCMD.rmw(|cmd| cmd & !CMD_SUTW);

            // FIXME is this needed?

            // repeat some of `usb_device` logic here because `usb_device` won't
            // trigger the STATUS out phase
            match &buf[..4] {
                // SET_ADDRESS -- no data phase
                [0, 5, _, _] => {}
                // SET_CONFIGURATION
                [0, 9, _, _] => {}
                // SET_INTERFACE
                [1, 11, _, _] => {}

                // GET_DESCRIPTOR
                [128, 6, _, _] => {
                    self.needs_status_out = true;
                }
                _ => {
                    memlog!("unexpected SETUP packet: {:?}", &buf[..n]);
                    memlog_flush_and_reset!();
                }
            }

            memlog!("... {:?} @ {:?}", &buf[..n], time::uptime());
            crate::memlog_try_flush();

            Ok(n)
        } else {
            // the dTD should already be installed in `next_dtd`
            unsafe {
                assert!(dqh.get_current_dtd().is_none());
                assert!(dqh.get_next_dtd().is_some());
            }

            // "Executing a transfer descriptor", section 54.4.6.6.3
            let dtd = unsafe { dqh.get_next_dtd().expect("UNREACHABLE") };
            let cap = buf.len();

            unsafe {
                let mut token = Token::empty();
                token.set_total_bytes(cap);
                token.set_status(Status::active());
                dtd.set_token(token);
                dtd.set_pages(buf.as_ptr());
            }

            // force all previous memory operations to complete before
            // priming
            atomic::fence(Ordering::Release);

            // "Prime endpoint by writing 1 to correct bit position in ENDPTPRIME"
            let mask = util::epaddr2endptmask(ep_addr);
            self.usb.ENDPTPRIME.rmw(|prime| prime | mask);

            // now the hardware can modify dQH and dTD
            memlog!("OUT{} primed @ {:?}", ep_addr.index(), time::uptime());

            // FIXME return WouldBlock instead of busy waiting
            // wait for completion
            if util::wait(
                || self.usb.ENDPTCOMPLETE.read() & mask != 0,
                2 * consts::microframe(),
            )
            .is_err()
            {
                memlog!("read: ENDPTCOMPLETE timeout");
                memlog_flush_and_reset!();
            }

            // synchronize with DMA operations before reading dQH or dTD
            atomic::fence(Ordering::Acquire);

            // clear complete bit
            self.usb.ENDPTCOMPLETE.write(mask);
            let token = unsafe { dtd.get_token() };
            let status = token.get_status();

            if status.is_active() || status.has_errors() || status.is_halted() {
                memlog!("read: DMA transfer failed");
                memlog_flush_and_reset!();
            }

            let left = unsafe { dqh.get_token().get_total_bytes() };
            let n = cap - left as usize;
            memlog!("... read {:?} @ {:?}", &buf[..n], time::uptime());

            // leave the dTD in place for the next transfer
            unsafe {
                dqh.clear_current_dtd();
                dqh.set_next_dtd(Some(dtd));
            }

            Ok(n)
        }
    }

    fn write(&mut self, ep_addr: EndpointAddress, bytes: &[u8]) -> Result<usize, UsbError> {
        memlog!(
            "write(ep_addr={:?}, bytes_len={}) ... @ {:?}",
            ep_addr,
            bytes.len(),
            time::uptime()
        );
        crate::memlog_try_flush();

        assert_eq!(ep_addr.index(), 0, "not yet implemented");

        let dqh = self.get_dqh(ep_addr).expect("UNREACHABLE");

        // "Executing a transfer descriptor", section 54.4.6.6.3
        // the dTD should already be installed in `next_dtd`
        unsafe {
            assert!(dqh.get_current_dtd().is_none());
            assert!(dqh.get_next_dtd().is_some());
        }

        // this is the first time this endpoint is being used
        let dtd = unsafe { dqh.get_next_dtd().expect("UNREACHABLE") };
        let n = bytes.len();

        unsafe {
            let mut token = Token::empty();
            token.set_total_bytes(n);
            token.set_status(Status::active());
            dtd.set_token(token);
            dtd.set_pages(bytes.as_ptr());
        }

        // force all previous memory operations to complete before
        // priming
        atomic::fence(Ordering::Release);

        // "Prime endpoint by writing 1 to correct bit position in ENDPTPRIME"
        let mask = util::epaddr2endptmask(ep_addr);
        self.usb.ENDPTPRIME.rmw(|prime| prime | mask);

        // now the hardware can modify dQH and dTD
        memlog!("IN{} primed @ {:?}", ep_addr.index(), time::uptime());

        // FIXME return WouldBlock instead of busy waiting
        // wait for completion
        if util::wait(
            || self.usb.ENDPTCOMPLETE.read() & mask != 0,
            2 * consts::microframe(),
        )
        .is_err()
        {
            memlog!("write: ENDPTCOMPLETE timeout");
            memlog_flush_and_reset!();
        }

        // synchronize with DMA operations before reading dQH or dTD
        atomic::fence(Ordering::Acquire);

        // clear complete bit
        self.usb.ENDPTCOMPLETE.write(mask);
        let token = unsafe { dtd.get_token() };
        let status = token.get_status();

        if status.is_active() || status.has_errors() || status.is_halted() {
            memlog!("write: DMA transfer failed");
            memlog_flush_and_reset!();
        }

        self.set_ep_in_complete(ep_addr.index());

        memlog!("... wrote {} bytes @ {:?}", bytes.len(), time::uptime());

        // leave the dTD in place for the next transfer
        unsafe {
            dqh.clear_current_dtd();
            dqh.set_next_dtd(Some(dtd));
        }

        Ok(n)
    }

    // # Helper functions
    fn port_change(&mut self) {
        memlog!("port_change @ {:?}", time::uptime());
        crate::memlog_try_flush();

        // clear the 'Port Change Detect' bit
        self.usb.USBSTS.write(USBSTS_PCI);

        // TODO do endpoint setup here?
        // TODO set up transfer descriptors
    }

    fn get_dqh(&self, ep_addr: EndpointAddress) -> Option<Ref<dQH>> {
        let dqhidx = util::epaddr2dqhidx(ep_addr);
        // bounds check
        if dqhidx < ENDPOINTS {
            // NOTE(unsafe) `ENDPTLISTADDR` has already been initialized at this point
            Some(unsafe {
                Ref::new_unchecked((self.usb.ENDPTLISTADDR.read() as *const dQH).add(dqhidx))
            })
        } else {
            None
        }
    }

    fn is_ep_being_used(&self, ep_addr: EndpointAddress) -> bool {
        let mask = 1 << util::epaddr2dqhidx(ep_addr);
        self.used_dqh & mask != 0
    }

    fn mark_ep_as_used(&mut self, ep_addr: EndpointAddress) {
        let mask = 1 << util::epaddr2dqhidx(ep_addr);
        self.used_dqh |= mask;
    }

    fn set_ep_in_complete(&mut self, index: usize) {
        assert!(index < ENDPOINTS / 2);

        if let Some(ep_in_complete) = self.ep_in_complete.as_mut() {
            *ep_in_complete |= 1 << index;
        } else {
            self.ep_in_complete = Some(1 << index);
        }
    }
}
