use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, Context};

use crate::codegen::util;

pub fn codegen(ctxt: Context<'_>, resources_tick: bool, app: &App) -> TokenStream2 {
    let mut items = vec![];
    let mut fields = vec![];
    let mut values = vec![];

    let name = ctxt.ident(app);

    let mut lt = None;

    if ctxt.has_locals(app) {
        let ident = util::locals_ident(ctxt, app);
        items.push(quote!(
            #[doc(inline)]
            pub use super::#ident as Locals;
        ));
    }

    if ctxt.has_resources(app) {
        let ident = util::resources_ident(ctxt, app);
        let lt = if resources_tick {
            lt = Some(quote!('a));
            Some(quote!('a))
        } else {
            None
        };

        items.push(quote!(
            #[doc(inline)]
            pub use super::#ident as Resources;
        ));

        fields.push(quote!(
            /// Resources this task has access to
            pub resources: Resources<#lt>
        ));

        let priority = if ctxt.is_init() {
            None
        } else {
            Some(quote!(priority))
        };
        values.push(quote!(resources: Resources::new(#priority)));
    }

    if ctxt.uses_spawn(app) {
        let doc = "Tasks that can be `spawn`-ed from this context";
        if ctxt.is_init() {
            fields.push(quote!(
                #[doc = #doc]
                pub spawn: Spawn
            ));

            items.push(quote!(
                #[doc = #doc]
                #[derive(Clone, Copy)]
                pub struct Spawn {
                    _not_send_or_sync: core::marker::PhantomData<*mut ()>,
                }
            ));

            values.push(quote!(spawn: Spawn { _not_send_or_sync: core::marker::PhantomData }));
        } else {
            lt = Some(quote!('a));

            fields.push(quote!(
                #[doc = #doc]
                pub spawn: Spawn<'a>
            ));

            if ctxt.is_idle() {
                items.push(quote!(
                    #[doc = #doc]
                    #[derive(Clone, Copy)]
                    pub struct Spawn<'a> {
                        priority: &'a rtic::export::Priority,
                    }
                ));

                values.push(quote!(spawn: Spawn { priority }));
            } else {
                items.push(quote!(
                    /// Tasks that can be spawned from this context
                    #[derive(Clone, Copy)]
                    pub struct Spawn<'a> {
                        priority: &'a rtic::export::Priority,
                    }
                ));

                values.push(quote!(
                    spawn: Spawn { priority }
                ));
            }

            items.push(quote!(
                impl<'a> Spawn<'a> {
                    #[doc(hidden)]
                    #[inline(always)]
                    pub unsafe fn priority(&self) -> &rtic::export::Priority {
                        self.priority
                    }
                }
            ));
        }
    }

    if let Context::Init(core) = ctxt {
        let init = &app.inits[&core];
        if init.returns_late_resources {
            let late_resources = util::late_resources_ident(&init.name);

            items.push(quote!(
                #[doc(inline)]
                pub use super::#late_resources as LateResources;
            ));
        }
    }

    let doc = match ctxt {
        Context::Idle(_) => "Idle loop",
        Context::Init(_) => "Initialization function",
        Context::HardwareTask(_) => "Hardware task",
        Context::SoftwareTask(_) => "Software task",
    };

    let priority = if ctxt.is_init() {
        None
    } else {
        Some(quote!(priority: &#lt rtic::export::Priority))
    };

    items.push(quote!(
        /// Execution context
        pub struct Context<#lt> {
            #(#fields,)*
        }

        impl<#lt> Context<#lt> {
            #[inline(always)]
            pub unsafe fn new(#priority) -> Self {
                Context {
                    #(#values,)*
                }
            }
        }
    ));

    if !items.is_empty() {
        quote!(
            #[allow(non_snake_case)]
            #[doc = #doc]
            pub mod #name {
                #(#items)*
            }
        )
    } else {
        quote!()
    }
}
