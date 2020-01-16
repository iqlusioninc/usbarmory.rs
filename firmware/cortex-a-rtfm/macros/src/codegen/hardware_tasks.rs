use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct};

pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_hardware_tasks -- interrupt handlers and `${task}Resources` constructors
    Vec<TokenStream2>,
    // root_hardware_tasks -- items that must be placed in the root of the crate:
    // - `${task}Locals` structs
    // - `${task}Resources` structs
    // - `${task}` modules
    Vec<TokenStream2>,
    // user_hardware_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {
    let mut const_app = vec![];
    let mut root = vec![];
    let mut user_tasks = vec![];

    for (name, task) in &app.hardware_tasks {
        let locals_new = if task.locals.is_empty() {
            quote!()
        } else {
            quote!(#name::Locals::new(),)
        };
        let symbol = &task.args.binds;
        let priority = task.args.priority;

        const_app.push(quote!(
            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe extern "C" fn #symbol() {
                const PRIORITY: u8 = #priority;

                rtfm::export::run(PRIORITY, || {
                    crate::#name(
                        #locals_new
                        #name::Context::new(&rtfm::export::Priority::new(PRIORITY))
                    )
                });
            }
        ));

        let mut needs_lt = false;

        // `${task}Resources`
        if !task.args.resources.is_empty() {
            let (item, constructor) = resources_struct::codegen(
                Context::HardwareTask(name),
                priority,
                &mut needs_lt,
                app,
                analysis,
            );

            root.push(item);

            const_app.push(constructor);
        }

        root.push(module::codegen(Context::HardwareTask(name), needs_lt, app));

        // `${task}Locals`
        let locals_pat = if !task.locals.is_empty() {
            let (struct_, pat) = locals::codegen(Context::HardwareTask(name), &task.locals, app);

            root.push(struct_);
            Some(pat)
        } else {
            None
        };

        let attrs = &task.attrs;
        let context = &task.context;
        let stmts = &task.stmts;
        let locals_pat = locals_pat.iter();
        user_tasks.push(quote!(
            #(#attrs)*
            #[allow(non_snake_case)]
            fn #name(#(#locals_pat,)* #context: #name::Context) {
                use rtfm::Mutex as _;

                #(#stmts)*
            }
        ));
    }

    (const_app, root, user_tasks)
}
