use proc_macro2::TokenStream;
use quote::quote;
use rtic_syntax::{analyze::Analysis, ast::App};

mod assertions;
mod dispatchers;
mod hardware_tasks;
mod idle;
mod init;
mod locals;
mod module;
mod post_init;
mod pre_init;
mod resources;
mod resources_struct;
mod software_tasks;
mod spawn;
mod spawn_body;
mod util;

#[allow(clippy::cognitive_complexity)]
pub fn app(app: &App, analysis: &Analysis) -> TokenStream {
    let assertion_stmts = assertions::codegen(analysis);

    let pre_init_stmts = pre_init::codegen(&app, analysis);

    let (const_app_init, root_init, user_init, call_init) = init::codegen(app, analysis);

    let post_init_stmts = post_init::codegen(analysis);

    let (const_app_idle, root_idle, user_idle, call_idle) = idle::codegen(app, analysis);

    let (const_app_resources, mod_resources) = resources::codegen(app, analysis);

    let (const_app_hardware_tasks, root_hardware_tasks, user_hardware_tasks) =
        hardware_tasks::codegen(app, analysis);

    let (const_app_software_tasks, root_software_tasks, user_software_tasks) =
        software_tasks::codegen(app, analysis);

    let const_app_dispatchers = dispatchers::codegen(app, analysis);

    let const_app_spawn = spawn::codegen(app);

    let name = &app.name;
    quote!(
        #user_init
        #user_idle
        #(#user_hardware_tasks)*
        #(#user_software_tasks)*

        #(#root_init)*
        #(#root_idle)*
        #(#root_hardware_tasks)*
        #(#root_software_tasks)*

        #mod_resources

        /// Implementation details -- the app author can't access the items
        /// defined inside this `const` item
        const #name: () = {
            #(#const_app_resources)*
            #const_app_init
            #const_app_idle
            #(#const_app_hardware_tasks)*
            #(#const_app_software_tasks)*
            #(#const_app_dispatchers)*
            #(#const_app_spawn)*

            #[no_mangle]
            unsafe extern "Rust" fn main() -> ! {
                #(#assertion_stmts)*

                #(#pre_init_stmts)*

                #call_init

                #(#post_init_stmts)*

                #call_idle
            }
        };
    )
}
