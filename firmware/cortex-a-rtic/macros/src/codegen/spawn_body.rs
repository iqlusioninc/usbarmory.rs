use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::{ast::App, Context};
use syn::Ident;

use crate::codegen::util;

pub fn codegen(spawner: Context<'_>, name: &Ident, app: &App) -> TokenStream2 {
    let spawnee = &app.software_tasks[name];
    let priority = spawnee.args.priority;

    let t = util::spawn_t_ident(priority);
    let fq = util::fq_ident(name);
    let rq = util::rq_ident(priority);

    let (dequeue, enqueue) = if spawner.is_init() {
        (
            quote!(#fq.dequeue()),
            quote!(#rq.enqueue_unchecked((#t::#name, index));),
        )
    } else {
        (
            quote!((#fq { priority }.lock(|fq| fq.split().1.dequeue()))),
            quote!((#rq { priority }.lock(|rq| {
                rq.split().0.enqueue_unchecked((#t::#name, index))
            }));),
        )
    };

    let sgi = util::prio2sgi(spawnee.args.priority);
    let (_, tupled, _, _) = util::regroup_inputs(&spawnee.inputs);
    let inputs = util::inputs_ident(name);
    quote!(
        unsafe {
            use rtic::Mutex as _;

            let input = #tupled;
            if let Some(index) = #dequeue {
                #inputs.get_unchecked_mut(usize::from(index)).as_mut_ptr().write(input);

                #enqueue

                rtic::export::pend_sgi(#sgi);

                Ok(())
            } else {
                Err(input)
            }
        }
    )
}
