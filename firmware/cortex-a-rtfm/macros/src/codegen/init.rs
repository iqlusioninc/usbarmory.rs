use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct, util};

/// Generates support code for `#[init]` functions
pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_idle -- the `${init}Resources` constructor
    Option<TokenStream2>,
    // root_init -- items that must be placed in the root of the crate:
    // - the `${init}Locals` struct
    // - the `${init}Resources` struct
    // - the `${init}LateResources` struct
    // - the `${init}` module, which contains types like `${init}::Context`
    Vec<TokenStream2>,
    // user_init -- the `#[init]` function written by the user
    Option<TokenStream2>,
    // call_init -- the call to the user `#[init]` if there's one
    Option<TokenStream2>,
) {
    const PRIORITY: u8 = 0;

    let mut root_init = vec![];

    if let Some(init) = app.inits.get(&crate::CORE) {
        let mut needs_lt = false;
        // context name
        let cname = &init.name;

        let sig_output = {
            let late_fields = analysis
                .late_resources
                .get(&crate::CORE)
                .map(|resources| {
                    resources
                        .iter()
                        .map(|rname| {
                            let ty = &app.late_resources[rname].ty;

                            quote!(pub #rname: #ty)
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or(vec![]);

            if !late_fields.is_empty() {
                let late_resources = util::late_resources_ident(cname);

                root_init.push(quote!(
                    /// Resources initialized at runtime
                    pub struct #late_resources {
                        #(#late_fields),*
                    }
                ));

                Some(quote!(-> #cname::LateResources))
            } else {
                None
            }
        };

        let mut locals_pat = None;
        let mut locals_new = None;
        if !init.locals.is_empty() {
            let (struct_, pat) = locals::codegen(Context::Init(crate::CORE), &init.locals, app);

            locals_new = Some(quote!(#cname::Locals::new()));
            locals_pat = Some(pat);
            root_init.push(struct_);
        }

        let context = &init.context;
        let attrs = &init.attrs;
        let stmts = &init.stmts;
        let locals_pat = locals_pat.iter();
        let user_init = Some(quote!(
            #(#attrs)*
            fn #cname(#(#locals_pat,)* #context: #cname::Context) #sig_output {
                #(#stmts)*
            }
        ));

        let mut const_app = None;
        if !init.args.resources.is_empty() {
            let (item, constructor) = resources_struct::codegen(
                Context::Init(crate::CORE),
                PRIORITY,
                &mut needs_lt,
                app,
                analysis,
            );

            root_init.push(item);
            const_app = Some(constructor);
        }

        let locals_new = locals_new.iter();
        let call_init = Some(quote!(let late = #cname(#(#locals_new,)* #cname::Context::new());));

        root_init.push(module::codegen(Context::Init(crate::CORE), needs_lt, app));

        (const_app, root_init, user_init, call_init)
    } else {
        (None, vec![], None, None)
    }
}
