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

        // ad-hoc: it's more convenient to split the SNVS in a Low-Power (LP)
        // domain peripheral and a High-Power(HP) peripheral
        if registers[0].name.starts_with("SNVS_") {
            const LP: &str = "SNVS_LP";
            const HP: &str = "SNVS_HP";

            let mut lp = vec![];
            let mut hp = vec![];
            for mut register in registers {
                if register.name.starts_with(LP) {
                    register.name =
                        Box::leak(Box::from(format!("{}_{}", LP, &register.name[LP.len()..])));
                    lp.push(register);
                } else if register.name.starts_with(HP) {
                    register.name =
                        Box::leak(Box::from(format!("{}_{}", HP, &register.name[HP.len()..])));
                    hp.push(register);
                } else {
                    panic!("unexpected SNVS register: {}", register.name);
                }
            }
            peripherals.extend(compress::registers(lp));
            peripherals.extend(compress::registers(hp));
        } else {
            let mut registers = registers;

            // ad-hoc: the base address of the DCP on the IMX6ULZ is not the same as on the IMX28
            if registers[0].name.starts_with("HW_DCP_") {
                const IMX28_BASE: u32 = 0x8002_8000;
                const IMX6ULZ_BASE: u32 = 0x0228_0000;

                for register in &mut registers {
                    register.abs_addr -= IMX28_BASE;
                    register.abs_addr += IMX6ULZ_BASE;
                }
            }

            peripherals.extend(compress::registers(registers));
        }
    }
    peripherals.push(gic::gicc());
    peripherals.push(gic::gicd());

    let krate = codegen::krate(&peripherals);

    fs::write("lib.rs", krate.to_string().into_bytes())?;
    Ok(())
}
