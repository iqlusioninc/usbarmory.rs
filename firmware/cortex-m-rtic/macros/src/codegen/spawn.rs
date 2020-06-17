use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rtfm_syntax::ast::App;

use crate::codegen::{spawn_body, util};

pub fn codegen(app: &App) -> Vec<TokenStream2> {
    let mut items = vec![];

    let mut seen = HashSet::new();
    for (spawner, spawnees) in app.spawn_callers() {
        // single-core sanity check
        assert_eq!(spawner.core(app), crate::CORE);

        let mut methods = vec![];

        for name in spawnees {
            let spawnee = &app.software_tasks[name];
            let cfgs = &spawnee.cfgs;
            let (args, _, untupled, ty) = util::regroup_inputs(&spawnee.inputs);
            let args = &args;

            if spawner.is_init() {
                // `init` uses a special spawn implementation; it doesn't use the `spawn_${name}`
                // functions which are shared by other contexts

                let body = spawn_body::codegen(spawner, &name, app);

                methods.push(quote!(
                    #(#cfgs)*
                    fn #name(&self #(,#args)*) -> Result<(), #ty> {
                        #body
                    }
                ));
            } else {
                let spawn = util::spawn_ident(name);

                if !seen.contains(name) {
                    // generate a `spawn_${name}` function
                    seen.insert(name);

                    let body = spawn_body::codegen(spawner, &name, app);

                    items.push(quote!(
                        #(#cfgs)*
                        unsafe fn #spawn(
                            priority: &rtfm::export::Priority
                            #(,#args)*
                        ) -> Result<(), #ty> {
                            #body
                        }
                    ));
                }

                methods.push(quote!(
                    #(#cfgs)*
                    #[inline(always)]
                    fn #name(&self #(,#args)*) -> Result<(), #ty> {
                        unsafe {
                            #spawn(self.priority() #(,#untupled)*)
                        }
                    }
                ));
            }
        }

        let lt = if spawner.is_init() {
            None
        } else {
            Some(quote!('a))
        };

        let spawner = spawner.ident(app);
        debug_assert!(!methods.is_empty());
        items.push(quote!(
            impl<#lt> #spawner::Spawn<#lt> {
                #(#methods)*
            }
        ));
    }

    items
}
