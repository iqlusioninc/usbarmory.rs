use proc_macro2::TokenStream;
use quote::quote;
use rtic_syntax::analyze::Analysis;

pub fn codegen(analysis: &Analysis) -> Vec<TokenStream> {
    let mut stmts = vec![];

    if let Some(resources) = analysis.late_resources.get(&crate::CORE) {
        for name in resources {
            // if it's live
            if analysis.locations.get(name).is_some() {
                stmts.push(quote!(#name.as_mut_ptr().write(late.#name);));
            }
        }
    }

    // enable the interrupts -- this completes the `init`-ialization phase
    stmts.push(quote!(rtic::export::enable_irq();));

    stmts
}
