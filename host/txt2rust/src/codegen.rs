use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::LitInt;

use crate::{
    codegen,
    compress::{Instances, Peripheral},
    Access,
};

pub fn krate(peripherals: &[Peripheral]) -> TokenStream2 {
    let mut items = vec![codegen::common()];

    for peripheral in peripherals {
        let pname_s = &peripheral.name;
        let pname_i = format_ident!("{}", pname_s);

        match &peripheral.instances {
            Instances::Single(base_addr) => {
                let mut mod_items = vec![];
                let mut fields = vec![];
                let mut initializers = vec![];

                for reg in &peripheral.registers {
                    let offset = reg.offset;
                    let name_i = format_ident!("{}", reg.name);
                    let uxx = format_ident!("u{}", reg.width);

                    let doc = if reg.instances == 1 {
                        reg.description.clone()
                    } else {
                        format!("{} ({} instances)", reg.description, reg.instances).into()
                    };
                    fields.push(quote!(
                        #[doc = #doc]
                        pub #name_i: #name_i
                    ));

                    initializers.push(quote!(
                        #name_i: #name_i {
                            _not_send_or_sync: PhantomData,
                        }
                    ));

                    let mut methods = vec![];

                    let (iarg, iparam, iassert, ioffset) = if reg.instances == 1 {
                        (quote!(), quote!(), quote!(), quote!())
                    } else {
                        let uxx = if reg.instances <= 1 << 8 {
                            quote!(u8)
                        } else {
                            quote!(u16)
                        };
                        let max = reg.instances;
                        (
                            quote!(idx ,),
                            quote!(idx: #uxx ,),
                            quote!(assert!(idx < #max as #uxx);),
                            quote!(.add(usize::from(idx))),
                        )
                    };

                    if reg.access != Access::WriteOnly {
                        methods.push(quote!(
                            /// Performs a single load operation on the memory-mapped register
                            pub fn read(&self , #iparam) -> #uxx {
                                #iassert
                                unsafe {
                                    ((BASE_ADDRESS + Self::OFFSET) as *const #uxx) #ioffset .read_volatile()
                                }
                            }
                        ));
                    }

                    if reg.access != Access::ReadOnly {
                        let (method, arg) = if reg.access == Access::WriteOneToClear {
                            (format_ident!("clear"), format_ident!("mask"))
                        } else {
                            (format_ident!("write"), format_ident!("bits"))
                        };

                        let unsafety = if reg.unsafe_write {
                            quote!(unsafe)
                        } else {
                            quote!()
                        };

                        methods.push(quote!(
                            /// Performs a single store operation on the memory-mapped register
                            #[allow(unused_unsafe)]
                            pub #unsafety fn #method(&self , #iparam #arg: #uxx) {
                                #iassert
                                unsafe {
                                    ((BASE_ADDRESS + Self::OFFSET) as *mut #uxx) #ioffset .write_volatile(#arg)
                                }
                            }
                        ));

                        if reg.access == Access::ReadWrite {
                            methods.push(quote!(
                                /// Performs a read-modify-write on the memory-mapped register
                                ///
                                /// This is a short-hand for `self.write(f(self.read()))`
                                #[allow(unused_unsafe)]
                                pub #unsafety fn rmw(
                                    &self , #iparam
                                    f: impl FnOnce(#uxx) -> #uxx,
                                ) {
                                    self.write(#iarg f(self.read(#iarg)))
                                }
                            ))
                        }
                    }

                    let reset_value = reg
                        .reset_value
                        .map(|rv| {
                            let rv = unsuffixed_hex(rv, false);

                            if reg.access == Access::ReadWrite || reg.access == Access::WriteOnly {
                                let unsafety = if reg.unsafe_write {
                                    quote!(unsafe)
                                } else {
                                    quote!()
                                };

                                methods.push(quote!(
                                    /// Writes the reset value
                                    #[allow(unused_unsafe)]
                                    pub #unsafety fn reset(
                                        &self , #iparam
                                    ) {
                                        self.write(#iarg Self::RESET_VALUE)
                                    }
                                ));
                            }

                            quote!(
                                /// Reset value
                                pub const RESET_VALUE: #uxx = #rv;
                            )
                        })
                        .unwrap_or_else(|| quote!());

                    let offset = unsuffixed_hex(offset, true);
                    mod_items.push(quote!(
                        #[doc = #doc]
                        #[allow(non_camel_case_types)]
                        pub struct #name_i {
                            _not_send_or_sync: PhantomData<*mut ()>,
                        }

                        impl #name_i {
                            const OFFSET: usize = #offset;
                            #reset_value

                            #(#methods)*
                        }
                    ));
                }

                let initializers = &initializers;

                mod_items.push(quote!(
                    impl Registers {
                        /// # Safety
                        ///
                        /// Creates a singleton from thin air; make sure we
                        /// never hand out two instances of it
                        unsafe fn new() -> Self {
                            Self {
                                _not_sync: PhantomData,
                                #(#initializers,)*
                            }
                        }
                    }
                ));

                mod_items.push(quote!(
                    #[allow(non_camel_case_types)]
                    #[doc = #pname_s]
                    pub type #pname_i = Registers;

                    impl #pname_i {
                        /// Takes the singleton that represents this peripheral instance
                        pub fn take() -> Option<Self> {
                            static TAKEN: AtomicBool = AtomicBool::new(false);

                            if TAKEN
                                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                                .is_ok()
                            {
                                Some(unsafe { Registers::new() })
                            } else {
                                None
                            }
                        }

                        /// Borrows the singleton without checking if it's
                        /// currently being held by a context
                        ///
                        /// **WARNING** this can break Read-Modify-Write
                        /// operations being performed in other contexts.
                        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
                            f(&Registers {
                                _not_sync: PhantomData,
                                #(#initializers,)*
                            })
                        }
                    }
                ));

                let base_addr = unsuffixed_hex(*base_addr, false);
                let mod_i = format_ident!("{}", pname_s.to_lowercase());
                items.push(quote!(
                    #[allow(non_snake_case)]
                    #[doc = #pname_s]
                    pub mod #mod_i {
                        use core::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};

                        const BASE_ADDRESS: usize = #base_addr;

                        /// The registers that make up the peripheral
                        #[allow(non_snake_case)]
                        pub struct Registers {
                            _not_sync: PhantomData<*mut ()>,
                            #(#fields,)*
                        }

                        unsafe impl Send for Registers {}

                        #(#mod_items)*
                    }
                ));
            }

            Instances::Many(instances) => {
                let mut mod_items = vec![];
                let mut fields = vec![];
                let mut initializers = vec![];

                for reg in &peripheral.registers {
                    let offset = reg.offset;
                    let name_i = format_ident!("{}", reg.name);
                    let uxx = format_ident!("u{}", reg.width);

                    let doc = if reg.instances == 1 {
                        reg.description.clone()
                    } else {
                        format!("{} ({} instances)", reg.description, reg.instances).into()
                    };
                    fields.push(quote!(
                        #[doc = #doc]
                        pub #name_i: #name_i<P>
                    ));

                    initializers.push(quote!(
                        #name_i: #name_i {
                            _p: PhantomData,
                            _not_send_or_sync: PhantomData,
                        }
                    ));

                    let mut methods = vec![];

                    let (iarg, iparam, iassert, ioffset) = if reg.instances == 1 {
                        (quote!(), quote!(), quote!(), quote!())
                    } else {
                        let uxx = if reg.instances <= 1 << 8 {
                            quote!(u8)
                        } else {
                            quote!(u16)
                        };
                        let max = reg.instances;
                        (
                            quote!(idx ,),
                            quote!(idx: #uxx ,),
                            quote!(assert!(idx < #max as #uxx);),
                            quote!(.add(usize::from(idx))),
                        )
                    };

                    if reg.access != Access::WriteOnly {
                        methods.push(quote!(
                            /// Performs a single load operation on the memory-mapped register
                            pub fn read(&self , #iparam) -> #uxx {
                                #iassert
                                unsafe {
                                    ((P::BASE_ADDRESS + Self::OFFSET) as *const #uxx) #ioffset .read_volatile()
                                }
                            }
                        ));
                    }

                    if reg.access != Access::ReadOnly {
                        let (method, arg) = if reg.access == Access::WriteOneToClear {
                            (format_ident!("clear"), format_ident!("mask"))
                        } else {
                            (format_ident!("write"), format_ident!("bits"))
                        };

                        let unsafety = if reg.unsafe_write {
                            quote!(unsafe)
                        } else {
                            quote!()
                        };

                        methods.push(quote!(
                            /// Performs a single store operation on the memory-mapped register
                            #[allow(unused_unsafe)]
                            pub #unsafety fn #method(&self , #iparam #arg: #uxx) {
                                #iassert
                                unsafe {
                                    ((P::BASE_ADDRESS + Self::OFFSET) as *mut #uxx) #ioffset .write_volatile(#arg)
                                }
                            }
                        ));

                        if reg.access == Access::ReadWrite {
                            methods.push(quote!(
                                /// Performs a read-modify-write on the memory-mapped register
                                ///
                                /// This is a short-hand for `self.write(f(self.read()))`
                                #[allow(unused_unsafe)]
                                pub #unsafety fn rmw(
                                    &self , #iparam
                                    f: impl FnOnce(#uxx) -> #uxx,
                                ) {
                                    self.write(#iarg f(self.read(#iarg)))
                                }
                            ))
                        }
                    }

                    let reset_value = reg
                        .reset_value
                        .map(|rv| {
                            let rv = unsuffixed_hex(rv, false);

                            if reg.access == Access::ReadWrite || reg.access == Access::WriteOnly {
                                let unsafety = if reg.unsafe_write {
                                    quote!(unsafe)
                                } else {
                                    quote!()
                                };

                                methods.push(quote!(
                                    /// Writes the reset value
                                    #[allow(unused_unsafe)]
                                    pub #unsafety fn reset(
                                        &self , #iparam
                                    ) {
                                        self.write(#iarg Self::RESET_VALUE)
                                    }
                                ));
                            }

                            quote!(
                                /// Reset value
                                pub const RESET_VALUE: #uxx = #rv;
                            )
                        })
                        .unwrap_or_else(|| quote!());

                    let offset = unsuffixed_hex(offset, true);
                    mod_items.push(quote!(
                        #[doc = #doc]
                        #[allow(non_camel_case_types)]
                        pub struct #name_i<P>
                        where
                            P: Peripheral,
                        {
                            _p: PhantomData<P>,
                            _not_send_or_sync: PhantomData<*mut ()>,
                        }

                        impl<P> #name_i<P>
                        where
                            P: Peripheral,
                        {
                            const OFFSET: usize = #offset;
                            #reset_value

                            #(#methods)*
                        }
                    ));
                }

                let initializers = &initializers;

                mod_items.push(quote!(
                    impl<P> Registers<P>
                    where
                        P: Peripheral,
                    {
                        /// # Safety
                        ///
                        /// Creates a singleton from thin air; make sure we
                        /// never hand out two instances of it
                        unsafe fn new() -> Self {
                            Self {
                                _p: PhantomData,
                                _not_sync: PhantomData,
                                #(#initializers,)*
                            }
                        }
                    }
                ));

                for (instance, base_addr) in instances {
                    let name_s = format!("{}{}", pname_s, instance);
                    let name_i = format_ident!("{}", name_s);
                    let n = format_ident!("_{}", instance);

                    let base_addr = unsuffixed_hex(*base_addr, false);
                    mod_items.push(quote!(
                        #[doc = #name_s]
                        pub struct #n;

                        impl Peripheral for #n {
                            const BASE_ADDRESS: usize = #base_addr;
                        }

                        #[allow(non_camel_case_types)]
                        #[doc = #name_s]
                        pub type #name_i = Registers<#n>;

                        impl #name_i {
                            /// Takes the singleton that represents this peripheral instance
                            pub fn take() -> Option<Self> {
                                static TAKEN: AtomicBool = AtomicBool::new(false);

                                if TAKEN
                                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                                    .is_ok()
                                {
                                    Some(unsafe { Registers::new() })
                                } else {
                                    None
                                }
                            }

                            /// Borrows the singleton without checking if it's
                            /// currently being held by a context
                            ///
                            /// **WARNING** this can break Read-Modify-Write
                            /// operations being performed in other contexts.
                            pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
                                f(unsafe { &Registers::new() })
                            }
                        }
                    ));
                }

                let mod_i = format_ident!("{}", pname_s.to_lowercase());
                items.push(quote!(
                    #[allow(non_snake_case)]
                    #[doc = #pname_s]
                    pub mod #mod_i {
                        use core::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};

                        use crate::Peripheral;

                        /// The registers that make up the peripheral
                        #[allow(non_snake_case)]
                        pub struct Registers<P>
                        where
                            P: Peripheral,
                        {
                            _p: PhantomData<P>,
                            _not_sync: PhantomData<*mut ()>,
                            #(#fields,)*
                        }

                        unsafe impl<P> Send for Registers<P> where P: Peripheral {}

                        #(#mod_items)*
                    }
                ));
            }
        }
    }

    quote!(
        #(#items)*
    )
}

pub fn common() -> TokenStream2 {
    quote!(
        //! Peripheral Access Crate
        //!
        //! Automatically generated. Do not directly modify the source code.

        #![no_std]
        #![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

        /// A peripheral instance
        pub trait Peripheral {
            /// The base address of this peripheral instance
            const BASE_ADDRESS: usize;
        }
    )
}

fn unsuffixed_hex(val: u32, compact: bool) -> LitInt {
    if compact {
        if val <= 0xff {
            return LitInt::new(&format!("{:#04x}", val), Span::call_site());
        } else if val <= 0xffff {
            return LitInt::new(&format!("{:#06x}", val), Span::call_site());
        }
    }

    let high = val >> 16;
    let low = val & 0xffff;
    LitInt::new(&format!("{:#06x}_{:04x}", high, low), Span::call_site())
}
