use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App, Context};

use crate::codegen::{locals, module, resources_struct, util};

pub fn codegen(
    app: &App,
    analysis: &Analysis,
) -> (
    // const_app_software_tasks -- free queues, buffers and `${task}Resources` constructors
    Vec<TokenStream2>,
    // root_software_tasks -- items that must be placed in the root of the crate:
    // - `${task}Locals` structs
    // - `${task}Resources` structs
    // - `${task}` modules
    Vec<TokenStream2>,
    // user_software_tasks -- the `#[task]` functions written by the user
    Vec<TokenStream2>,
) {
    let mut const_app = vec![];
    let mut root = vec![];
    let mut user_tasks = vec![];

    for (name, task) in &app.software_tasks {
        let inputs = &task.inputs;
        let (_, _, _, input_ty) = util::regroup_inputs(inputs);

        let cap = task.args.capacity;
        let cap_lit = util::capacity_literal(cap);
        let cap_ty = util::capacity_typenum(cap, true);

        // create free queues and inputs buffers
        if let Some(free_queues) = analysis.free_queues.get(name) {
            for (&sender, &ceiling) in free_queues {
                // single-core sanity check
                assert_eq!(sender, crate::CORE);

                let fq = util::fq_ident(name);

                let fq_ty = quote!(rtfm::export::FQ<#cap_ty>);
                let fq_expr = quote!(rtfm::export::Queue(rtfm::export::iQueue::u8()));

                const_app.push(quote!(
                    /// Queue version of a free-list that keeps track of empty slots in
                    /// the following buffers
                    static mut #fq: #fq_ty = #fq_expr;
                ));

                // Generate a resource proxy if needed
                if let Some(ceiling) = ceiling {
                    const_app.push(quote!(
                        struct #fq<'a> {
                            priority: &'a rtfm::export::Priority,
                        }
                    ));

                    const_app.push(util::impl_mutex(
                        &[],
                        false,
                        &fq,
                        fq_ty.clone(),
                        ceiling,
                        quote!(&mut #fq),
                    ));
                }

                let elems = &(0..cap)
                    .map(|_| quote!(core::mem::MaybeUninit::uninit()))
                    .collect::<Vec<_>>();

                let uninit = util::link_section_uninit();
                let inputs = util::inputs_ident(name);
                const_app.push(quote!(
                    #uninit
                    /// Buffer that holds the inputs of a task
                    static mut #inputs: [core::mem::MaybeUninit<#input_ty>; #cap_lit] =
                        [#(#elems,)*];
                ));
            }
        }

        // `${task}Resources`
        let mut needs_lt = false;
        if !task.args.resources.is_empty() {
            let (item, constructor) = resources_struct::codegen(
                Context::SoftwareTask(name),
                task.args.priority,
                &mut needs_lt,
                app,
                analysis,
            );

            root.push(item);

            const_app.push(constructor);
        }

        // `${task}Locals`
        let mut locals_pat = None;
        if !task.locals.is_empty() {
            let (struct_, pat) = locals::codegen(Context::SoftwareTask(name), &task.locals, app);

            locals_pat = Some(pat);
            root.push(struct_);
        }

        let context = &task.context;
        let attrs = &task.attrs;
        let cfgs = &task.cfgs;
        let stmts = &task.stmts;
        let locals_pat = locals_pat.iter();
        user_tasks.push(quote!(
            #(#attrs)*
            #(#cfgs)*
            #[allow(non_snake_case)]
            fn #name(#(#locals_pat,)* #context: #name::Context #(,#inputs)*) {
                use rtfm::Mutex as _;

                #(#stmts)*
            }
        ));

        root.push(module::codegen(Context::SoftwareTask(name), needs_lt, app));
    }

    (const_app, root, user_tasks)
}
