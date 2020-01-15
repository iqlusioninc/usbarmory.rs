use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::{analyze::Analysis, ast::App};

use crate::codegen::util;

pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    // NOTE on reset IRQ interrupts are disabled

    for (name, senders) in &analysis.free_queues {
        let task = &app.software_tasks[name];
        let cap = task.args.capacity;

        for &_sender in senders.keys() {
            // single-core sanity check
            assert_eq!(_sender, crate::CORE);

            let fq = util::fq_ident(name);

            stmts.push(quote!(
                (0..#cap).for_each(|i| #fq.enqueue_unchecked(i));
            ));
        }
    }

    // enable the GIC
    stmts.push(quote!(rtfm::export::enable_gic();));

    // set the priority mask to its lowest (logical) value
    let logical_prio = crate::IDLE_PRIORITY;
    stmts.push(quote!(rtfm::export::set_priority_mask(#logical_prio);));

    // set SGI priorities
    let sgis_in_use = app
        .software_tasks
        .iter()
        .map(|(_, task)| task.args.priority)
        .collect::<HashSet<_>>()
        .len();
    for sgi in 0..sgis_in_use as u8 {
        let logical_prio = util::sgi2prio(sgi);
        stmts.push(quote!(
            rtfm::export::set_sgi_priority(#sgi, #logical_prio);
        ));
    }

    stmts
}
