use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct};

pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_idle -- the `${idle}Resources` constructor
    Option<TokenStream2>,
    // root_idle -- items that must be placed in the root of the crate:
    // - the `${idle}Locals` struct
    // - the `${idle}Resources` struct
    // - the `${idle}` module, which contains types like `${idle}::Context`
    Vec<TokenStream2>,
    // user_idle
    Option<TokenStream2>,
    // call_idle
    TokenStream2,
) {
    const PRIORITY: u8 = 0;

    if let Some(idle) = app.idles.get(&crate::CORE) {
        let mut needs_lt = false;
        let mut const_app = None;
        let mut root_idle = vec![];
        let mut locals_pat = None;
        let mut locals_new = None;

        if !idle.args.resources.is_empty() {
            let (item, constructor) = resources_struct::codegen(
                Context::Idle(crate::CORE),
                PRIORITY,
                &mut needs_lt,
                app,
                analysis,
            );

            root_idle.push(item);
            const_app = Some(constructor);
        }

        // context name
        let cname = &idle.name;
        if !idle.locals.is_empty() {
            let (locals, pat) = locals::codegen(Context::Idle(crate::CORE), &idle.locals, app);

            locals_new = Some(quote!(#cname::Locals::new()));
            locals_pat = Some(pat);
            root_idle.push(locals);
        }

        root_idle.push(module::codegen(Context::Idle(crate::CORE), needs_lt, app));

        let attrs = &idle.attrs;
        let context = &idle.context;
        let stmts = &idle.stmts;
        let locals_pat = locals_pat.iter();
        let user_idle = Some(quote!(
            #(#attrs)*
            fn #cname(#(#locals_pat,)* #context: #cname::Context) -> ! {
                use rtic::Mutex as _;

                #(#stmts)*
            }
        ));

        let locals_new = locals_new.iter();
        let call_idle = quote!(#cname(
            #(#locals_new,)*
            #cname::Context::new(&rtic::export::Priority::new(#PRIORITY))
        ));

        (const_app, root_idle, user_idle, call_idle)
    } else {
        (
            None,
            vec![],
            None,
            quote!(loop {
                // FIXME the processor is not waking up from WFI
                // rtic::export::wfi()
            }),
        )
    }
}
