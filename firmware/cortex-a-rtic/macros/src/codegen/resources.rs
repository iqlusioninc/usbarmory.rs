use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{
    analyze::{Analysis, Ownership},
    ast::App,
};

use crate::codegen::util;

pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app -- the `static [mut]` variables behind the proxies
    Vec<TokenStream2>,
    // mod_resources -- the `resources` module
    TokenStream2,
) {
    let mut const_app = vec![];
    let mut mod_resources = vec![];

    for (name, res, src_expr, _loc) in app.resources(analysis) {
        let cfgs = &res.cfgs;
        // user type
        let uty = &res.ty;

        // ity = internal type
        let (ity, expr) = if let Some(expr) = src_expr {
            (quote!(#uty), quote!(#expr))
        } else {
            (
                quote!(core::mem::MaybeUninit<#uty>),
                quote!(core::mem::MaybeUninit::uninit()),
            )
        };

        let attrs = &res.attrs;
        const_app.push(quote!(
            #[allow(non_upper_case_globals)]
            #(#attrs)*
            #(#cfgs)*
            static mut #name: #ity = #expr;
        ));

        if let Some(Ownership::Contended { ceiling }) = analysis.ownerships.get(name) {
            mod_resources.push(quote!(
                #[allow(non_camel_case_types)]
                #(#cfgs)*
                pub struct #name<'a> {
                    priority: &'a Priority,
                }

                #(#cfgs)*
                impl<'a> #name<'a> {
                    #[inline(always)]
                    pub unsafe fn new(priority: &'a Priority) -> Self {
                        #name { priority }
                    }

                    #[inline(always)]
                    pub unsafe fn priority(&self) -> &Priority {
                        self.priority
                    }
                }
            ));

            let ptr = if src_expr.is_none() {
                quote!(#name.as_mut_ptr())
            } else {
                quote!(&mut #name)
            };

            const_app.push(util::impl_mutex(
                cfgs,
                true,
                name,
                quote!(#uty),
                *ceiling,
                ptr,
            ));
        }
    }

    let mod_resources = if mod_resources.is_empty() {
        quote!()
    } else {
        quote!(mod resources {
            use rtic::export::Priority;

            #(#mod_resources)*
        })
    };

    (const_app, mod_resources)
}
