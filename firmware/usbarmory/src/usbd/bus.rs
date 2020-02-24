//! `UsbBus` implementation

use core::{
    mem,
    ptr::{self, NonNull},
    slice,
    sync::atomic::{self, Ordering},
};

use usb_device::{
    bus::{PollResult, UsbBus},
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection, UsbError,
};

use super::{
    dqh::dQH,
    token::{Status, Token},
    util::{self, Data, OneIndices, Ref},
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
        self.inner
            .lock(|inner| inner.alloc_ep(ep_dir, ep_addr, ep_type, max_packet_size, interval))
    }

    fn enable(&mut self) {
        self.inner.lock(|inner| inner.enable());
    }

    fn is_stalled(&self, _: EndpointAddress) -> bool {
        false
    }

    fn poll(&self) -> PollResult {
        self.inner.lock(|inner| inner.poll())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        self.inner.lock(|inner| inner.read(ep_addr, buf))
    }

    fn reset(&self) {
        self.inner.lock(|inner| inner.reset());
    }

    fn resume(&self) {
        // TODO do something in the `resume` callback
    }

    fn set_stalled(&self, _: EndpointAddress, stalled: bool) {
        if stalled {
            // FIXME handle stall conditions
            unimplemented!()
        }
    }

    fn suspend(&self) {
        // TODO do something in the `suspend` callback
    }

    fn set_device_address(&self, addr: u8) {
        self.inner.lock(|inner| inner.set_device_address(addr));
    }

    fn write(&self, ep_addr: EndpointAddress, bytes: &[u8]) -> Result<usize, UsbError> {
        self.inner.lock(|inner| inner.start_write(ep_addr, bytes))
    }
}

/// USB Reset Received
const USBSTS_URI: u32 = 1 << 6;

/// Start of Frame (SoF) received interrupt
const USBSTS_SRE: u32 = 1 << 7;

/// Port Change Detect
const USBSTS_PCI: u32 = 1 << 2;

/// TX Endpoint Enable
const ENDPTCTRL_TXE: u32 = 1 << 23;

/// TX Data Toggle Reset
const ENDPTCTRL_TXR: u32 = 1 << 22;

/// RX Endpoint Enable
const ENDPTCTRL_RXE: u32 = 1 << 7;

/// RX Data Toggle Reset
const ENDPTCTRL_RXR: u32 = 1 << 6;

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
            "alloc_ep(ep_dir={:?}, ep={:?}, ep_type={:?}, max_packet_size={}, interval={}) @ {:?}",
            ep_dir,
            ep_addr.map(|ep| ep.index()),
            ep_type,
            max_packet_size,
            interval,
            time::uptime(),
        );
        assert_ne!(ep_type, EndpointType::Isochronous, "not supported");

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

        // NOTE(unsafe) hardware cannot yet access the dQH and dTD
        unsafe {
            dqh.set_max_packet_size(max_packet_size, true);

            // install a dTD for the endpoint
            let dtd = Ref::new(self.dtds.pop().expect("exhausted the dTD pool"));

            if ep_addr.is_out() && ep_addr.index() != 0 {
                // install buffer in the dTD
                let addr = NonNull::new_unchecked(if max_packet_size <= 64 {
                    self.b64s
                        .pop()
                        .expect("OOM during 64-byte buffer request")
                        .as_mut_ptr()
                } else if max_packet_size <= 512 {
                    self.b512s
                        .pop()
                        .expect("OOM during 64-byte buffer request")
                        .as_mut_ptr()
                } else {
                    unimplemented!("buffers of {}-bytes are not available", max_packet_size)
                });

                let mut token = Token::empty();
                token.set_total_bytes(max_packet_size.into());
                token.set_status(Status::active());
                token.set_ioc();
                dtd.set_token(token);
                dtd.set_pages(addr);
                dqh.set_address(addr);
            }

            dqh.set_next_dtd(Some(dtd));
        }

        // NOTE no memory barrier here because we are not going to hand this to
        // the hardware just yet
        drop(dqh);

        // mark this endpoint as used
        self.mark_ep_as_used(ep_addr);
        // NOTE we should do endpoint configuration after the device has
        // transitioned from the 'Address' state to the 'Configured' state but
        // `usb_device` provides no hook to do that so we just do it here.
        if ep_addr.index() != 0 {
            self.configure_ep(ep_addr, ep_type);
        }

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
        if util::wait_for_or_timeout(|| self.usb.ENDPTPRIME.read() == 0, 2 * consts::frame())
            .is_err()
        {
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
        // TODO report `Suspend` and `Resume` events

        /// When a controller enters a suspend state from an active state
        const USBSTS_SLI: u32 = 1 << 8;
        /// System error
        const USBSTS_SEI: u32 = 1 << 4;

        // The Start of Frame (SoF) event will trigger the interrupt handler
        // roughly every 125 us. The SoF is synchronized to USB events. The
        // interrupt flag must be cleared to avoid missing the next SoF event
        self.usb.USBSTS.write(USBSTS_SRE);

        let sts = self.usb.USBSTS.read();

        if sts & USBSTS_URI != 0 {
            memlog!("poll() -> Reset @ {:?}", time::uptime());
            crate::memlog_try_flush();

            self.last_poll_was_none = false;
            return PollResult::Reset;
        }

        let setupstat = self.usb.ENDPTSETUPSTAT.read() as u16;
        let mut complete = self.usb.ENDPTCOMPLETE.read();

        if sts & USBSTS_PCI != 0 {
            self.port_change();
        }

        if setupstat != 0 {
            // cache `setuptstat`; it needs special handling in `read`
            self.setupstat = Some(setupstat);
        }

        let txcomplete = complete >> 16;
        if txcomplete != 0 {
            for bit in OneIndices::of(txcomplete) {
                self.end_write(bit);
            }

            complete &= 0xffff;
        }

        if setupstat != 0 || self.ep_in_complete.is_some() || self.status_out != 0 || complete != 0
        {
            let ep_setup = setupstat;
            let ep_in_complete = self.ep_in_complete.take().unwrap_or(0);
            // STATUS out needs to be reported after the IN data phase
            let ep_out = if self.status_out != 0 && ep_in_complete == 0 {
                mem::replace(&mut self.status_out, 0)
            } else {
                // the higher bits were cleared in the previous `if` block
                complete as u16
            };

            let data = Data {
                ep_in_complete,
                ep_setup,
                ep_out,
            };

            memlog!("poll() -> {:?} @ {:?}", data, time::uptime());
            crate::memlog_try_flush();

            self.last_poll_was_none = false;
            return data.into();
        }

        if !self.last_poll_was_none {
            self.last_poll_was_none = true;
            memlog!("poll() -> None");
        }
        crate::memlog_try_flush();

        PollResult::None
    }

    fn read(&mut self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        memlog!(
            "read(ep={}, cap={}, self.setupstat={:?}) ... @ {:?}",
            ep_addr.index(),
            buf.len(),
            self.setupstat,
            time::uptime()
        );
        crate::memlog_try_flush();

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

        if self.setupstat == Some(0) {
            self.setupstat = None;
        }

        let dqh = self.get_dqh(ep_addr).ok_or(UsbError::InvalidEndpoint)?;
        let ep_mask = util::epaddr2endptmask(ep_addr);
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
            self.clear_interrupt();

            // repeat some of `usb_device` logic here because `usb_device` won't
            // trigger the STATUS out phase nor does it have hook for
            // SET_CONFIGURATION
            match &buf[..4] {
                // SET_ADDRESS -- no data phase
                [0, 5, _, _] => {}
                // SET_CONFIGURATION
                [0, 9, _, _] |
                // SET_INTERFACE
                [1, 11, _, _] => {
                    // FIXME (a) we should only reset the endpoints when the
                    // configuration changed. (b) we should only reset the
                    // endpoints that are part of the new configuration
                    for ep_addr in self.allocated_eps() {
                        self.reset_ep(ep_addr)
                    }
                }

                // GET_DESCRIPTOR
                [128, 6, _, _] => {
                    self.pre_status_out = 1;
                }
                _ => {
                    memlog!("unexpected SETUP packet: {:?}", &buf[..n]);
                    memlog_flush_and_reset!();
                }
            }

            memlog!("... {:?} @ {:?}", &buf[..n], time::uptime());
            crate::memlog_try_flush();

            Ok(n)
        } else if ep_addr.index() == 0 {
            // FIXME can we set up the buffer and prime the endpoint earlier?

            // the dTD should already be installed in `next_dtd`
            // TODO turn into debug_assertions
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
                token.set_ioc();
                dtd.set_token(token);
                let addr = NonNull::new_unchecked(buf.as_ptr() as *mut u8);
                dtd.set_pages(addr);
                dqh.set_address(addr);
            }

            // force all previous memory operations to complete before
            // priming
            atomic::fence(Ordering::Release);

            // prime the endpoint
            self.usb.ENDPTPRIME.rmw(|prime| prime | ep_mask);

            // now the hardware can modify dQH and dTD
            memlog!("OUT{} primed @ {:?}", ep_addr.index(), time::uptime());

            // FIXME return WouldBlock instead of busy waiting
            // wait for completion
            if util::wait_for_or_timeout(
                || self.usb.ENDPTCOMPLETE.read() & ep_mask != 0,
                2 * consts::microframe(),
            )
            .is_err()
            {
                memlog!("read: ENDPTCOMPLETE timeout");
                memlog_flush_and_reset!();
            }

            // synchronize with DMA operations before reading dQH or dTD
            atomic::fence(Ordering::Acquire);

            // TODO invalidate the data cache before reading `dtd`

            // clear complete bit
            self.usb.ENDPTCOMPLETE.write(ep_mask);
            self.clear_interrupt();
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
        } else {
            if self.usb.ENDPTCOMPLETE.read() & ep_mask == 0 {
                return Err(UsbError::WouldBlock);
            }

            // copy out the data and re-prime buffer
            let dtd = unsafe { dqh.get_current_dtd().expect("UNREACHABLE") };

            // clear complete bit
            self.usb.ENDPTCOMPLETE.write(ep_mask);
            self.clear_interrupt();

            // synchronize with DMA operations before reading dQH or dTD
            atomic::fence(Ordering::Acquire);

            // TODO invalidate the data cache before reading `dtd`

            let token = unsafe { dtd.get_token() };
            let status = token.get_status();

            if status.is_active() || status.has_errors() || status.is_halted() {
                memlog!("read: DMA transfer failed");
                memlog_flush_and_reset!();
            }

            // TODO get `total_bytes` from `dtd` after invalidating the cache
            let left = unsafe { dqh.get_token().get_total_bytes() };
            let max_packet_size = dqh.get_max_packet_size();
            let n = max_packet_size - left;
            // NOTE OUT endpoints are given a buffer during `alloc_ep`
            let addr = dqh.get_address().expect("UNREACHABLE");

            unsafe {
                buf[..n.into()]
                    .copy_from_slice(slice::from_raw_parts(addr.as_ptr(), usize::from(n)));
            }

            memlog!("read: {} bytes @ {:?}", n, time::uptime());

            unsafe {
                let mut token = Token::empty();
                token.set_total_bytes(max_packet_size.into());
                token.set_status(Status::active());
                token.set_ioc();
                dtd.set_token(token);
                dtd.set_pages(addr);

                // leave the dTD in place for the next transfer
                dqh.clear_current_dtd();
                dqh.set_next_dtd(Some(dtd));
            }

            // force all previous memory operations to complete before
            // priming
            atomic::fence(Ordering::Release);

            // prime the endpoint
            self.usb.ENDPTPRIME.rmw(|prime| prime | ep_mask);

            memlog!("OUT{} primed @ {:?}", ep_addr.index(), time::uptime());

            Ok(n.into())
        }
    }

    fn start_write(&mut self, ep_addr: EndpointAddress, bytes: &[u8]) -> Result<usize, UsbError> {
        memlog!(
            "start_write(ep={}, bytes_len={}) ... @ {:?}",
            ep_addr.index(),
            bytes.len(),
            time::uptime()
        );
        crate::memlog_try_flush();

        let dqh = self.get_dqh(ep_addr).ok_or(UsbError::InvalidEndpoint)?;
        let max_packet_size = dqh.get_max_packet_size();
        let n = bytes.len();

        if n > usize::from(max_packet_size) {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        // "Executing a transfer descriptor", section 54.4.6.6.3
        // the dTD should already be installed in `next_dtd`
        unsafe {
            if dqh.get_current_dtd().is_some() {
                // transfer in progress
                return Err(UsbError::WouldBlock);
            } else {
                assert!(dqh.get_next_dtd().is_some());
            }
        }

        // this is the first time this endpoint is being used
        let dtd = unsafe { dqh.get_next_dtd().expect("UNREACHABLE") };

        let addr = if let Some(addr) = dqh.get_address() {
            addr
        } else {
            let addr = unsafe {
                NonNull::new_unchecked(if max_packet_size <= 64 {
                    self.b64s.pop().expect("OOM").as_mut_ptr()
                } else if max_packet_size <= 512 {
                    self.b512s.pop().expect("OOM").as_mut_ptr()
                } else {
                    unimplemented!()
                })
            };

            dqh.set_address(addr);
            addr
        };

        unsafe {
            let mut token = Token::empty();
            token.set_total_bytes(n);
            token.set_status(Status::active());
            token.set_ioc();
            dtd.set_token(token);
            // copy data into static buffer
            ptr::copy_nonoverlapping(bytes.as_ptr(), addr.as_ptr(), n);
            dtd.set_pages(addr);
            dqh.set_address(addr);
        }

        // force all previous memory operations to complete before
        // priming
        atomic::fence(Ordering::Release);

        // "Prime endpoint by writing 1 to correct bit position in ENDPTPRIME"
        let mask = util::epaddr2endptmask(ep_addr);
        self.usb.ENDPTPRIME.rmw(|prime| prime | mask);

        // now the hardware can modify dQH and dTD
        memlog!("IN{} primed @ {:?}", ep_addr.index(), time::uptime());

        Ok(n)
    }

    fn end_write(&mut self, idx: u8) {
        let ep_addr = EndpointAddress::from_parts(usize::from(idx), UsbDirection::In);
        let mask = util::epaddr2endptmask(ep_addr);
        let dqh = self.get_dqh(ep_addr).expect("UNREACHABLE");
        let dtd = unsafe { dqh.get_current_dtd().expect("UNREACHABLE") };

        // synchronize with DMA operations before reading dQH or dTD
        atomic::fence(Ordering::Acquire);

        // TODO invalidate the cache before reading `dtd`

        // clear complete bit
        self.usb.ENDPTCOMPLETE.write(mask);
        self.clear_interrupt();
        let token = unsafe { dtd.get_token() };
        let status = token.get_status();

        if status.is_active() || status.has_errors() || status.is_halted() {
            memlog!("write: DMA transfer failed");
            memlog_flush_and_reset!();
        }

        self.set_ep_in_complete(ep_addr.index());

        let mask = (mask >> 16) as u16;
        if self.pre_status_out & mask != 0 {
            self.status_out |= mask;
            self.pre_status_out &= !mask;
        }

        memlog!("end_write(ep={}) @ {:?}", ep_addr.index(), time::uptime());

        // leave the dTD in place for the next transfer
        unsafe {
            dqh.clear_current_dtd();
            dqh.set_next_dtd(Some(dtd));
        }
    }

    fn set_device_address(&mut self, addr: u8) {
        memlog!("set_device_address({})", addr);
        crate::memlog_try_flush();

        // "instantaneous" address update
        self.usb.DEVICEADDR.write((addr as u32) << 25);

        // FIXME enabling an endpoint should be doing after receiving a
        // SET_CONFIGURATION control packet -- `usb_device` provides no hook for
        // that but we could do this in `read`
        for ep_addr in self.allocated_eps() {
            self.enable_ep(ep_addr)
        }
    }

    // # Helper functions
    /// Clears the USBSTS_UI bit
    fn clear_interrupt(&mut self) {
        /// USB Interrupt
        const USBSTS_UI: u32 = 1;

        self.usb.USBSTS.write(USBSTS_UI);
    }

    fn port_change(&mut self) {
        memlog!("port_change @ {:?}", time::uptime());
        crate::memlog_try_flush();

        // clear the 'Port Change Detect' bit
        self.usb.USBSTS.write(USBSTS_PCI);
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
        self.used_dqhs & mask != 0
    }

    fn mark_ep_as_used(&mut self, ep_addr: EndpointAddress) {
        let mask = 1 << util::epaddr2dqhidx(ep_addr);
        self.used_dqhs |= mask;
    }

    /// Returns an iterator over all the allocated endpoints
    fn allocated_eps(&self) -> impl Iterator<Item = EndpointAddress> {
        OneIndices::of(u32::from(self.used_dqhs)).map(|idx| util::dqhidx2epaddr(usize::from(idx)))
    }

    fn configure_ep(&mut self, ep_addr: EndpointAddress, ep_type: EndpointType) {
        assert_ne!(
            ep_addr.index(),
            0,
            "endpoint 0 is always the control endpoint"
        );

        let idx = ep_addr.index();
        assert_eq!(
            idx, 1,
            "configuring endpoint {} is not supported at the moment",
            idx
        );

        const ENDPTCTRL_TXT_OFFSET: u8 = 18;
        const ENDPTCTRL_TXT_MASK: u32 = 0b11;

        const ENDPTCTRL_RXT_OFFSET: u8 = 2;
        const ENDPTCTRL_RXT_MASK: u32 = 0b00;

        const BULKT: u32 = 0b10;
        let ty = match ep_type {
            EndpointType::Control => 0b00,
            EndpointType::Isochronous => 0b01,
            EndpointType::Bulk => BULKT,
            EndpointType::Interrupt => 0b11,
        };

        // "If one endpoint direction is enabled and the paired endpoint
        // ofopposite direction is disabled then the unused direction
        // typemust be changed from the default control-type to any other
        // type (that is Bulk-type). leaving an unconfigured endpoint
        // controlcauses undefined behavior for the data pid tracking on
        // the active endpoint/direction." -- section 56.6.40 of ULLRM
        if ep_addr.is_out() {
            self.usb.ENDPTCTRL1.rmw(|ctrl| {
                (ctrl & !(ENDPTCTRL_RXT_MASK << ENDPTCTRL_RXT_OFFSET))
                    | (ty << ENDPTCTRL_RXT_OFFSET)
            });

            if self.usb.ENDPTCTRL1.read() & ENDPTCTRL_TXE == 0 {
                self.usb.ENDPTCTRL1.rmw(|ctrl| {
                    (ctrl & !(ENDPTCTRL_TXT_MASK << ENDPTCTRL_TXT_OFFSET))
                        | (BULKT << ENDPTCTRL_TXT_OFFSET)
                });
            }
        } else {
            self.usb.ENDPTCTRL1.rmw(|ctrl| {
                (ctrl & !(ENDPTCTRL_TXT_MASK << ENDPTCTRL_TXT_OFFSET))
                    | (ty << ENDPTCTRL_TXT_OFFSET)
            });

            if self.usb.ENDPTCTRL1.read() & ENDPTCTRL_RXE == 0 {
                self.usb.ENDPTCTRL1.rmw(|ctrl| {
                    (ctrl & !(ENDPTCTRL_RXT_MASK << ENDPTCTRL_RXT_OFFSET))
                        | (BULKT << ENDPTCTRL_RXT_OFFSET)
                });
            }
        }
    }

    fn enable_ep(&mut self, ep_addr: EndpointAddress) {
        // TODO generalize beyond endpoint 1
        if ep_addr.is_out() {
            let idx = ep_addr.index();
            if idx != 0 {
                // prime the endpoint
                let mask = util::epaddr2endptmask(ep_addr);
                self.usb.ENDPTPRIME.rmw(|prime| prime | mask);

                memlog!("primed OUT{} @ {:?}", idx, time::uptime());
            }

            self.usb.ENDPTCTRL1.rmw(|ctrl| ctrl | ENDPTCTRL_RXE);
        } else {
            self.usb.ENDPTCTRL1.rmw(|ctrl| ctrl | ENDPTCTRL_TXE);
        }
    }

    /// Resets the endpoint PID sequence
    fn reset_ep(&mut self, ep_addr: EndpointAddress) {
        // TODO generalize beyond endpoint 1
        if ep_addr.is_out() {
            // TODO turn into a debug assertion
            assert_ne!(
                self.usb.ENDPTCTRL1.read() & ENDPTCTRL_RXE,
                0,
                "endpoint not enabled"
            );

            self.usb.ENDPTCTRL1.rmw(|ctrl| ctrl | ENDPTCTRL_RXR);
        } else {
            // TODO turn into a debug assertion
            assert_ne!(self.usb.ENDPTCTRL1.read() & ENDPTCTRL_TXE, 0);

            self.usb.ENDPTCTRL1.rmw(|ctrl| ctrl | ENDPTCTRL_TXR);
        }
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
