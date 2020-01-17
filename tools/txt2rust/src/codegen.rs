use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::{
    codegen,
    compress::{Instances, Peripheral},
    Access,
};

pub fn krate(peripherals: &[Peripheral]) -> TokenStream2 {
    let mut items = vec![codegen::common()];

    for peripheral in peripherals {
        let name = format_ident!("{}", peripheral.name);

        match &peripheral.instances {
            Instances::Single(base_addr) => {
                let mut mod_items = vec![];
                let mut fields = vec![];
                let mut initializers = vec![];

                for reg in &peripheral.registers {
                    let offset = reg.offset;
                    let name = format_ident!("{}", reg.name);
                    let uxx = format_ident!("u{}", reg.width);

                    fields.push(quote!(
                        pub #name: #name
                    ));

                    initializers.push(quote!(
                        #name: #name {
                            _not_send_or_sync: PhantomData,
                        }
                    ));

                    let mut methods = vec![];

                    if reg.access != Access::Write {
                        methods.push(quote!(
                            pub fn read(&self) -> #uxx {
                                unsafe {
                                    ((BASE_ADDRESS + Self::OFFSET) as *const #uxx).read_volatile()
                                }
                            }
                        ));
                    }

                    if reg.access != Access::Read {
                        let (name, arg) = if reg.access == Access::WriteOneToClear {
                            (format_ident!("clear"), format_ident!("mask"))
                        } else {
                            (format_ident!("write"), format_ident!("bits"))
                        };
                        methods.push(quote!(
                            pub fn #name(&self, #arg: #uxx) {
                                unsafe {
                                    ((BASE_ADDRESS + Self::OFFSET) as *mut #uxx).write_volatile(#arg)
                                }
                            }
                        ));
                    }

                    let doc = &reg.description;
                    mod_items.push(quote!(
                        #[doc = #doc]
                        #[allow(non_camel_case_types)]
                        pub struct #name {
                            _not_send_or_sync: PhantomData<*mut ()>,
                        }

                        impl #name {
                            const OFFSET: usize = #offset as usize;

                            #(#methods)*
                        }
                    ));
                }

                let initializers = &initializers;
                let name = format_ident!("{}", name);

                mod_items.push(quote!(
                    pub type #name = Registers;

                    impl #name {
                        pub fn take() -> Option<Self> {
                            static TAKEN: AtomicBool = AtomicBool::new(false);

                            if TAKEN
                                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                                .is_ok()
                            {
                                Some(Registers {
                                    _not_sync: PhantomData,
                                    #(#initializers,)*
                                })
                            } else {
                                None
                            }
                        }
                    }
                ));

                items.push(quote!(
                    #[allow(non_snake_case)]
                    pub mod #name {
                        use core::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};

                        const BASE_ADDRESS: usize = #base_addr as usize;

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
                    let name = format_ident!("{}", reg.name);
                    let uxx = format_ident!("u{}", reg.width);

                    fields.push(quote!(
                        pub #name: #name<P>
                    ));

                    initializers.push(quote!(
                        #name: #name {
                            _p: PhantomData,
                            _not_send_or_sync: PhantomData,
                        }
                    ));

                    let mut methods = vec![];

                    if reg.access != Access::Write {
                        methods.push(quote!(
                            pub fn read(&self) -> #uxx {
                                unsafe {
                                    ((P::BASE_ADDRESS + Self::OFFSET) as *const #uxx).read_volatile()
                                }
                            }
                        ));
                    }

                    if reg.access != Access::Read {
                        let (name, arg) = if reg.access == Access::WriteOneToClear {
                            (format_ident!("clear"), format_ident!("mask"))
                        } else {
                            (format_ident!("write"), format_ident!("bits"))
                        };
                        methods.push(quote!(
                            pub fn #name(&self, #arg: #uxx) {
                                unsafe {
                                    ((P::BASE_ADDRESS + Self::OFFSET) as *mut #uxx).write_volatile(#arg)
                                }
                            }
                        ));
                    }

                    let doc = &reg.description;
                    mod_items.push(quote!(
                        #[doc = #doc]
                        #[allow(non_camel_case_types)]
                        pub struct #name<P>
                        where
                            P: Peripheral,
                        {
                            _p: PhantomData<P>,
                            _not_send_or_sync: PhantomData<*mut ()>,
                        }

                        impl<P> #name<P>
                        where
                            P: Peripheral,
                        {
                            const OFFSET: usize = #offset as usize;

                            #(#methods)*
                        }
                    ));
                }

                let initializers = &initializers;
                for (instance, base_addr) in instances {
                    let name = format_ident!("{}{}", name, instance);
                    let n = format_ident!("_{}", instance);

                    mod_items.push(quote!(
                        pub struct #n;

                        impl Peripheral for #n {
                            const BASE_ADDRESS: usize = #base_addr as usize;
                        }

                        pub type #name = Registers<#n>;

                        impl #name {
                            pub fn take() -> Option<Self> {
                                static TAKEN: AtomicBool = AtomicBool::new(false);

                                if TAKEN
                                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                                    .is_ok()
                                {
                                    Some(Registers {
                                        _p: PhantomData,
                                        _not_sync: PhantomData,
                                        #(#initializers,)*
                                    })
                                } else {
                                    None
                                }
                            }
                        }
                    ));
                }

                items.push(quote!(
                    #[allow(non_snake_case)]
                    pub mod #name {
                        use core::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};

                        use crate::Peripheral;

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
        pub trait Peripheral {
            const BASE_ADDRESS: usize;
        }
    )
}
