use std::{env, fs};

pub mod codegen;
pub mod compress;
pub mod parse;

#[derive(Debug, PartialEq)]
pub enum Access {
    Read,
    ReadWrite,
    Write,
    WriteOneToClear,
}

fn main() -> Result<(), anyhow::Error> {
    let mut peripherals = vec![];
    for path in env::args().skip(1) {
        let file: &'static str = Box::leak(Box::<str>::from(fs::read_to_string(path)?));

        peripherals.extend(compress::registers(parse::register_table(file)));
    }

    let krate = codegen::krate(&peripherals);

    fs::write("lib.rs", krate.to_string().into_bytes())?;
    Ok(())
}
