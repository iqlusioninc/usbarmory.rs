use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use quote::{format_ident, quote};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = &PathBuf::from(env::var("OUT_DIR")?);
    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let target = env::var("TARGET")?;

    txt2rust(&out_dir)?;

    // place the linker script somewhere the linker can find it

    let link_x = if env::var("CARGO_FEATURE_USE_DRAM").is_ok() {
        fs::read("link-dram.x")?
    } else {
        fs::read("link-ocram.x")?
    };
    fs::write(out_dir.join("link.x"), link_x)?;

    // place the assembly part of the entry point somewhere the linker can find it
    fs::copy(
        format!("bin/{}.a", target),
        out_dir.join(format!("lib{}.a", pkg_name)),
    )?;
    println!("cargo:rustc-link-lib=static={}", pkg_name);

    println!("cargo:rustc-link-search={}", out_dir.display());

    Ok(())
}

struct Interrupt<'a> {
    name: &'a str,
    irq: u16,
    description: &'a str,
}

/// Generate Rust code from `interrupts.txt`
// NOTE `interrupts.txt` was generated running `pdftotex -layout -f $start -l
// $end` on the reference manual (`$start..$end` includes section 3.2), plus
// some manual edits to remove duplicated names
fn txt2rust(out_dir: &Path) -> Result<(), Box<dyn Error>> {
    const IRQ_START: u16 = 32;
    const NIRQS: usize = 128;

    let txt = fs::read_to_string("interrupts.txt")?;

    let mut entries = vec![];
    for line in txt.trim().lines() {
        const EOF: &str = "unexpected EOF";

        let mut parts = line.splitn(2, char::is_whitespace);
        let irq = parts
            .next()
            .ok_or("expected IRQ number; found EOF")?
            .parse()?;
        let mut parts = parts
            .next()
            .ok_or(EOF)?
            .trim()
            .splitn(2, char::is_whitespace);
        let name = parts
            .next()
            .ok_or("expected the interrupt name; found EOF")?;
        let description = parts
            .next()
            .ok_or("expected the description; found EOF")?
            .trim();

        entries.push(Interrupt {
            name,
            irq,
            description,
        });
    }

    if entries.len() != NIRQS {
        return Err("`interrupts.txt` must have 128 entries".into());
    }

    // remove all "reserved" entries
    let interrupts = entries
        .into_iter()
        .filter(|entry| entry.name.to_lowercase() != "reserved")
        .collect::<Vec<_>>();

    // Generate `enum Interrupt`
    let mut items = vec![];
    let variants = interrupts
        .iter()
        .map(|interrupt| {
            let description = interrupt.description;
            let name = format_ident!("{}", interrupt.name);
            let irq = interrupt.irq;

            quote!(
                #[doc = #description]
                #name = #irq,
            )
        })
        .collect::<Vec<_>>();

    items.push(quote!(
        /// List of interrupts
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        #[repr(u16)]
        pub enum Interrupt {
            #(#variants)*
        }

        impl Interrupt {
            /// Returns the interrupt IRQ number
            pub fn irq(self) -> u16 {
                self as u16
            }
        }
    ));

    let mut elmts = vec![];
    let mut pos = IRQ_START;
    for interrupt in &interrupts {
        while pos != interrupt.irq {
            // add a reserved entry
            elmts.push(quote!({
                extern "C" {
                    fn DefaultHandler();
                }

                DefaultHandler
            }));

            pos += 1;
        }

        let name = format_ident!("{}", interrupt.name);
        elmts.push(quote!(
            {
                extern "C" {
                    fn #name();
                }

                #name
            }
        ));
        pos += 1;
    }

    // Generate `SPIS` array
    items.push(quote!(
        static SPIS: [unsafe extern "C" fn(); #NIRQS] = [#(#elmts,)*];
    ));

    let code = quote!(#(#items)*);
    fs::write(out_dir.join("interrupts.rs"), code.to_string().into_bytes())?;

    // Also generate a linker script that provides a default value for all these interrupts
    let mut script = String::new();
    for interrupt in interrupts {
        script.push_str(&format!("PROVIDE({} = DefaultHandler);\n", interrupt.name));
    }
    fs::write(out_dir.join("interrupts.x"), script)?;

    Ok(())
}
