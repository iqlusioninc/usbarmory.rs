use std::collections::HashSet;

use proc_macro2::Span;
use rtfm_syntax::ast::App;
use syn::parse;

pub fn app(app: &App) -> parse::Result<()> {
    // check that there are enough SGIs (Software Generated Interrupts) to
    // dispatch the software tasks
    let priorities = app
        .software_tasks
        .iter()
        .map(|(_, task)| task.args.priority)
        .collect::<HashSet<_>>();

    let needed_sgis = priorities.len();
    if needed_sgis > crate::NSGIS {
        return Err(parse::Error::new(
            Span::call_site(),
            format!(
                "cannot dispatch more than {} different priority levels",
                crate::NSGIS
            ),
        ));
    }

    Ok(())
}
