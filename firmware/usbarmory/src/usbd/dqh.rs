use core::{
    cell::{Cell, UnsafeCell},
    fmt, ptr,
    sync::atomic::{self, Ordering},
};

use super::{
    dtd::{dTD, NO_NEXT_DTD},
    token::Token,
    util::{Ref, Reserved},
};

const SETUP_BYTES: usize = 8;

/// Endpoint Queue Header
// Memory layout is specified in table 54-56 of ULRM
// NOTE(align) "The dQH is a 48-byte data structure, but must be aligned on
// 64-byte boundaries"
// NOTE all instances of this struct are shared with the hardware; instances of
// this type should NEVER appear behind an exclusive reference (`&mut-`)
#[allow(non_camel_case_types)]
#[repr(align(64))]
#[repr(C)]
pub struct dQH {
    caps: Cell<Caps>,
    // NOTE these fields can be modified by the hardware. Care needs to be
    // exercised when accessing these fields. In particular, memory barriers are
    // required to avoid data races with the hardware and cache invalidation is
    // required to read the actual state of these fields.
    current_dtd: UnsafeCell<usize>,
    next_dtd: UnsafeCell<usize>,
    token: UnsafeCell<Token>,
    pages: [UnsafeCell<usize>; 5],
    // must always be zero
    reserved: UnsafeCell<Reserved>,
    // SETUP packets (see USB control transfers) are written to this filed
    setup: [UnsafeCell<u8>; SETUP_BYTES],

    // XXX I think we are allowed to use the 16 bytes that follow and are used
    // for padding (the hardware requires that dQHs are laid down in an array)
    addr: Cell<usize>,
}

impl dQH {
    pub const SETUP_BYTES: usize = SETUP_BYTES;

    pub const fn new() -> Self {
        dQH {
            caps: Cell::new(Caps::empty()),
            current_dtd: UnsafeCell::new(1),
            next_dtd: UnsafeCell::new(1),
            token: UnsafeCell::new(Token::empty()),
            pages: [
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
            ],
            reserved: UnsafeCell::new(Reserved::new()),
            setup: [
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
                UnsafeCell::new(0),
            ],
            addr: Cell::new(0),
        }
    }

    /// Clears all the fields
    ///
    /// # Safety
    ///
    /// Must be called only when the hardware is not operating on the dQH. Must
    /// be synchronized with a memory barrier before letting the hardware access
    /// this field
    pub unsafe fn clear(&self) {
        self.caps.set(Caps::empty());
        self.current_dtd.get().write(1);
    }

    // # Getters / Setters
    /// # Safety
    ///
    /// Must be called only when the hardware is not operating on the dQH. Must
    /// be synchronized with a memory barrier before letting the hardware read
    /// this field
    pub unsafe fn set_max_packet_size(&self, max_packet_size: u16) {
        self.caps.set(Caps::new(max_packet_size));
    }

    pub fn get_max_packet_size(&self) -> u16 {
        self.caps.get().max_packet_size()
    }

    pub fn get_address(&self) -> *const u8 {
        self.addr.get() as *const u8
    }

    pub fn set_address(&self, addr: *const u8) {
        self.addr.set(addr as usize)
    }

    /// # Safety
    ///
    /// The returned reference must be destroyed before the hardware is able to
    /// modify the dQH value
    pub unsafe fn get_current_dtd(&self) -> Option<Ref<dTD>> {
        let p = self.current_dtd.get().read();
        if p & 0b11111 != 0 {
            // garbage is currently stored in this field -- i.e. the hardware
            // has not yet written a value to it
            None
        } else {
            Some(Ref::new_unchecked(p as *const dTD))
        }
    }

    /// # Safety
    ///
    /// The returned reference must be destroyed before the hardware is able to
    /// modify the dQH value
    pub unsafe fn get_next_dtd(&self) -> Option<Ref<dTD>> {
        let p = self.next_dtd.get().read();
        if p == NO_NEXT_DTD {
            // terminate bit is set
            None
        } else {
            Some(Ref::new_unchecked(p as *const dTD))
        }
    }

    pub unsafe fn get_token(&self) -> Token {
        self.token.get().read()
    }

    /// # Safety
    ///
    /// The returned reference must be destroyed before the hardware is able to
    /// modify the dQH value
    pub unsafe fn clear_current_dtd(&self) {
        self.current_dtd.get().write(NO_NEXT_DTD);
    }

    /// # Safety
    ///
    /// The returned reference must be destroyed before the hardware is able to
    /// modify the dQH value
    pub unsafe fn set_next_dtd(&self, dtd: Option<Ref<dTD>>) {
        self.next_dtd
            .get()
            .write(dtd.map(|dtd| dtd.as_ptr() as usize).unwrap_or(NO_NEXT_DTD));
    }

    // # Higher level operations
    /// NOTE the hardware may change the setup bytes while this operation is in
    /// progress. Check USBCMD.SUTW to see if that race occurred or not
    pub fn copy_setup_bytes(&self, buf: &mut [u8]) {
        // NOTE(compiler_fence) prevent this non-volatile memory operation from
        // being reordered wrt to any other surrounding memory operation
        atomic::compiler_fence(Ordering::SeqCst);
        unsafe {
            ptr::copy_nonoverlapping(self.setup[0].get(), buf.as_mut_ptr(), Self::SETUP_BYTES);
        }
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

// See table 54-57 of the ULRM
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Caps {
    inner: u32,
}

impl Caps {
    const fn empty() -> Self {
        Self { inner: 0 }
    }

    /// Initializes the `Caps` with a configuration valid for non Isochronous
    /// endpoints
    pub fn new(max_packet_size: u16) -> Self {
        assert!(max_packet_size <= 0x400);

        // "Non-ISO endpoints must set Mult='00'" -- section 54.4.5.1.1
        let mult = 0b00;
        let zlt = 0;

        Self {
            inner: (mult << 30) | (zlt << 29) | (u32::from(max_packet_size) << 16),
        }
    }

    pub fn max_packet_size(self) -> u16 {
        (self.inner >> 16) as u16
    }
}

impl fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Caps")
            .field("max_packet_size", &self.max_packet_size())
            .finish()
    }
}

impl fmt::Debug for Ref<dQH> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let current_dtd = self.current_dtd.get().read();
            let current_dtd = if current_dtd == NO_NEXT_DTD {
                None
            } else {
                Some(Ref::new_unchecked(current_dtd as *const dTD))
            };

            let next_dtd = self.next_dtd.get().read();
            let next_dtd = if next_dtd == NO_NEXT_DTD {
                None
            } else {
                Some(Ref::new_unchecked(next_dtd as *const dTD))
            };

            f.debug_struct("dQH")
                .field("caps", &self.caps.get())
                .field("current_dtd", &current_dtd)
                .field("next_dtd", &next_dtd)
                .field("token", &self.token.get().read())
                .field(
                    "pages",
                    &[
                        self.pages[0].get().read(),
                        self.pages[1].get().read(),
                        self.pages[2].get().read(),
                        self.pages[3].get().read(),
                        self.pages[4].get().read(),
                    ],
                )
                .field(
                    "setup",
                    &[
                        self.setup[0].get().read(),
                        self.setup[1].get().read(),
                        self.setup[2].get().read(),
                        self.setup[3].get().read(),
                        self.setup[4].get().read(),
                        self.setup[5].get().read(),
                        self.setup[6].get().read(),
                        self.setup[7].get().read(),
                    ],
                )
                .finish()
        }
    }
}
