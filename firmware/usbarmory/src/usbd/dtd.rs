//! Transfer descriptors

use core::{
    cell::{Cell, UnsafeCell},
    fmt,
};

use super::{token::Token, util::Ref};

pub const NO_NEXT_DTD: usize = 1;

/// Device Transfer Descriptor (dTD)
// Layout is specified in table 54-60 of the ULRM
// NOTE all instances of this struct are shared with the hardware; instances of
// this type should NEVER appear behind an exclusive reference (`&mut-`)
// NOTE(align) Table 54-60 of the ULRM shows that a pointer into this data
// structure must be a multiple of 32
#[allow(non_camel_case_types)]
#[repr(C)]
#[repr(align(32))]
pub struct dTD {
    next_dtd: Cell<usize>,

    // NOTE the hardware can modify these 3 fields
    token: UnsafeCell<Token>,
    page0: UnsafeCell<usize>,
    page1: UnsafeCell<usize>,

    pages: [Cell<usize>; 3],
}

impl dTD {
    pub const fn new() -> Self {
        Self {
            next_dtd: Cell::new(NO_NEXT_DTD),

            token: UnsafeCell::new(Token::empty()),
            page0: UnsafeCell::new(0),
            page1: UnsafeCell::new(0),
            pages: [Cell::new(0), Cell::new(0), Cell::new(0)],
        }
    }

    /// # Safety
    /// - The hardware reads this field so it should not be modified while the
    ///   hardware is allowed to read it
    /// - the validity and lifetime of `ptr` must be verified by the caller
    pub unsafe fn set_pages(&self, ptr: *const u8) {
        let mut addr = ptr as usize;
        self.page0.get().write(addr);
        addr &= !0xfff;
        addr += 0x1000;
        self.page1.get().write(addr);
        for page in self.pages.iter() {
            addr += 0x1000;
            page.set(addr);
        }
    }

    /// # Safety
    /// The hardware reads this field so it should not be modified while the
    /// hardware is allowed to read it
    pub unsafe fn set_token(&self, token: Token) {
        self.token.get().write(token)
    }

    /// # Safety
    /// The hardware reads this field so it should not be modified while the
    /// hardware is allowed to read it
    pub unsafe fn set_next_dtd(&self, dtd: Option<Ref<dTD>>) {
        self.next_dtd
            .set(dtd.map(|dtd| dtd.as_ptr() as usize).unwrap_or(NO_NEXT_DTD))
    }

    /// # Safety
    /// The hardware modifies this field so it should not be read while the
    /// hardware is allowed to read it
    pub unsafe fn get_token(&self) -> Token {
        self.token.get().read()
    }
}

impl fmt::Debug for Ref<dTD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let next_dtd = self.next_dtd.get();
            let next_dtd = if next_dtd == NO_NEXT_DTD {
                None
            } else {
                Some(Ref::new_unchecked(next_dtd as *const dTD))
            };

            f.debug_struct("dTD")
                .field("next_dtd", &next_dtd)
                .field("token", &self.token.get().read())
                .field(
                    "pages",
                    &[
                        self.page0.get().read(),
                        self.page1.get().read(),
                        self.pages[0].get(),
                        self.pages[1].get(),
                        self.pages[2].get(),
                    ],
                )
                .finish()
        }
    }
}
