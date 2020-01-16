//! Real Time For the Masses `#[app]` attribute

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use std::{fs, path::Path};

use proc_macro::TokenStream;
use rtfm_syntax::{Core, Settings};

mod check;
mod codegen;

/// (hard-coded) Number of Software Generated Interrupts (SGIs) available
const NSGIS: usize = 16;

/// Idle priority (logical priority); lowest "priority" supported by the hardware
const IDLE_PRIORITY: u8 = 0;

/// Single-core implementation
const CORE: Core = 0;

/// Specifies a Real Time For the Masses application
#[proc_macro_attribute]
pub fn app(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut settings = Settings::default();
    settings.optimize_priorities = true;
    settings.parse_binds = true;

    let (app, analysis) = match rtfm_syntax::parse(args, input, settings) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    if let Err(e) = check::app(&app) {
        return e.to_compile_error().into();
    }

    let ts = codegen::app(&app, &analysis);

    // try to write the expanded code to disk
    if Path::new("target").exists() {
        fs::write("target/rtfm-expansion.rs", ts.to_string()).ok();
    }

    ts.into()
}
