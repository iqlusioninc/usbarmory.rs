use std::{env, fs};

pub mod codegen;
pub mod compress;
pub mod gic;
pub mod parse;

#[derive(Debug, PartialEq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    WriteOnly,
    WriteOneToClear,
}

fn main() -> Result<(), anyhow::Error> {
    let mut peripherals = vec![];
    for path in env::args().skip(1) {
        let file: &'static str = Box::leak(Box::<str>::from(fs::read_to_string(&path)?));

        let registers = parse::register_table(file);
        anyhow::ensure!(
            !registers.is_empty(),
            "{:?} appears to contain no register table",
            path
        );
        peripherals.extend(compress::registers(registers));
    }
    peripherals.push(gic::gicc());
    peripherals.push(gic::gicd());

    let krate = codegen::krate(&peripherals);

    fs::write("lib.rs", krate.to_string().into_bytes())?;
    Ok(())
}
