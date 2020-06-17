use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtic_syntax::analyze::Analysis;

pub fn codegen(analysis: &Analysis) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    if let Some(types) = analysis.send_types.get(&crate::CORE) {
        for ty in types {
            stmts.push(quote!(rtic::export::assert_send::<#ty>();));
        }
    }

    if let Some(types) = analysis.sync_types.get(&crate::CORE) {
        for ty in types {
            stmts.push(quote!(rtic::export::assert_sync::<#ty>();));
        }
    }

    stmts
}
