use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = &PathBuf::from(env::var("OUT_DIR")?);
    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let target = env::var("TARGET")?;

    // place the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), fs::read("link.x")?)?;

    // place the assembly part of the entry point somewhere the linker can find it
    fs::copy(
        format!("bin/{}.a", target),
        out_dir.join(format!("lib{}.a", pkg_name)),
    )?;
    println!("cargo:rustc-link-lib=static={}", pkg_name);

    println!("cargo:rustc-link-search={}", out_dir.display());

    Ok(())
}
