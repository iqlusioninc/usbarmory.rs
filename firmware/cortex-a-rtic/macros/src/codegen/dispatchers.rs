use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{analyze::Analysis, ast::App};

use crate::codegen::util;

pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    let mut items = vec![];

    for (&_receiver, dispatchers) in &analysis.channels {
        // single-core sanity check
        assert_eq!(_receiver, crate::CORE);

        for (&level, channels) in dispatchers {
            let mut stmts = vec![];

            for (&_sender, channel) in channels {
                // single-core sanity check
                assert_eq!(_sender, crate::CORE);

                let variants = channel
                    .tasks
                    .iter()
                    .map(|name| {
                        let cfgs = &app.software_tasks[name].cfgs;

                        quote!(
                            #(#cfgs)*
                            #name
                        )
                    })
                    .collect::<Vec<_>>();

                let doc = format!(
                    "Software tasks to be dispatched at priority level {}",
                    level,
                );

                let t = util::spawn_t_ident(level);
                items.push(quote!(
                    #[allow(non_camel_case_types)]
                    #[derive(Clone, Copy)]
                    #[doc = #doc]
                    enum #t {
                        #(#variants,)*
                    }
                ));

                let n = util::capacity_typenum(channel.capacity, true);
                let rq = util::rq_ident(level);
                let rq_ty = quote!(rtic::export::RQ<#t, #n>);
                let rq_expr = quote!(rtic::export::Queue(rtic::export::iQueue::u8()));

                let doc = format!(
                    "Queue of tasks ready to be dispatched at priority level {}",
                    level
                );
                items.push(quote!(
                    #[doc = #doc]
                    static mut #rq: #rq_ty = #rq_expr;
                ));

                if let Some(ceiling) = channel.ceiling {
                    items.push(quote!(
                        struct #rq<'a> {
                            priority: &'a rtic::export::Priority,
                        }
                    ));

                    items.push(util::impl_mutex(
                        &[],
                        false,
                        &rq,
                        rq_ty,
                        ceiling,
                        quote!(&mut #rq),
                    ));
                }

                let arms = channel
                    .tasks
                    .iter()
                    .map(|name| {
                        let task = &app.software_tasks[name];
                        let cfgs = &task.cfgs;
                        let fq = util::fq_ident(name);
                        let inputs = util::inputs_ident(name);
                        let (_, tupled, pats, _) = util::regroup_inputs(&task.inputs);

                        let locals_new = if task.locals.is_empty() {
                            quote!()
                        } else {
                            quote!(#name::Locals::new(),)
                        };

                        quote!(
                            #(#cfgs)*
                            #t::#name => {
                                let #tupled =
                                    #inputs.get_unchecked(usize::from(index)).as_ptr().read();
                                #fq.split().0.enqueue_unchecked(index);
                                let priority = &rtic::export::Priority::new(PRIORITY);
                                #name(
                                    #locals_new
                                    #name::Context::new(priority)
                                    #(,#pats)*
                                )
                            }
                        )
                    })
                    .collect::<Vec<_>>();

                stmts.push(quote!(
                    while let Some((task, index)) = #rq.split().1.dequeue() {
                        match task {
                            #(#arms)*
                        }
                    }
                ));
            }

            let doc = format!(
                "Software Generated Interrupt (SGI) handler used to dispatch tasks at priority {}",
                level
            );
            let sgi = util::sgi_ident(util::prio2sgi(level));
            // NOTE we use the Rust ABI here because the IRQ handler (written in
            // Rust) expects that ABI and can inline these
            items.push(quote!(
                #[allow(non_snake_case)]
                #[doc = #doc]
                #[inline(never)]
                #[no_mangle]
                unsafe extern "C" fn #sgi() {
                    /// The priority of this interrupt handler
                    const PRIORITY: u8 = #level;

                    rtic::export::run(PRIORITY, || {
                        #(#stmts)*
                    });
                }
            ));
        }
    }

    items
}
