use core::mem;
use std::{borrow::Cow, collections::BTreeMap};

use crate::{parse, Access};

#[derive(Debug)]
pub struct Register<'a> {
    pub offset: u32,
    pub description: Cow<'a, str>,
    pub name: &'a str,
    /// Width in bits
    pub width: u8,
    pub access: Access,
    pub reset_value: u32,
}

impl PartialEq for Register<'_> {
    fn eq(&self, rhs: &Register<'_>) -> bool {
        // we ignore the `description` field
        self.offset == rhs.offset
            && self.name == rhs.name
            && self.access == rhs.access
            && self.reset_value == rhs.reset_value
    }
}

#[derive(Debug)]
pub enum Instances {
    Single(/* base address */ u32),
    Many(BTreeMap</* instance index */ u8, /* base address */ u32>),
}

#[derive(Debug)]
pub struct Peripheral<'a> {
    pub name: &'a str,
    pub instances: Instances,
    pub registers: Vec<Register<'a>>,
}

pub fn registers(mut all_registers: Vec<parse::Register>) -> Vec<Peripheral> {
    all_registers.sort_by_key(|register| register.abs_addr);

    let (name, numbered) = find_common_prefix(all_registers[0].name, all_registers[1].name);

    if numbered {
        let mut peripherals = Vec::<Peripheral>::new();
        while !all_registers.is_empty() {
            // e.g. `7`
            let instance = extract_instance(all_registers[0].name.trim_start_matches(name));
            let base_addr = all_registers[0].abs_addr;
            // e.g. `UART7_`
            let instance_prefix = format!("{}{}_", name, instance);

            // registers are sorted by address; all the registers that belong to
            // the same peripheral instance should be one next to each other
            let idx = all_registers
                .iter()
                .position(|reg| !reg.name.starts_with(&instance_prefix));

            let registers = if let Some(idx) = idx {
                let rest = all_registers.split_off(idx);

                mem::replace(&mut all_registers, rest)
            } else {
                // this is the last instance
                mem::replace(&mut all_registers, vec![])
            };

            let registers = registers
                .into_iter()
                .map(|reg| Register {
                    offset: reg.abs_addr - base_addr,
                    description: reg.description,
                    name: reg.name.trim_start_matches(&instance_prefix),
                    width: reg.width,
                    access: reg.access,
                    reset_value: reg.reset_value,
                })
                .collect::<Vec<_>>();

            if let Some(idx) = peripherals
                .iter()
                .position(|periph| periph.registers == registers)
            {
                match &mut peripherals[idx].instances {
                    Instances::Many(map) => {
                        map.insert(instance, base_addr);
                    }

                    Instances::Single(..) => unreachable!(),
                }
            } else {
                let mut map = BTreeMap::new();
                map.insert(instance, base_addr);
                peripherals.push(Peripheral {
                    name,
                    instances: Instances::Many(map),
                    registers,
                });
            }
        }

        peripherals
    } else {
        let base_addr = all_registers[0].abs_addr;

        let registers = all_registers
            .into_iter()
            .map(|reg| Register {
                offset: reg.abs_addr - base_addr,
                description: reg.description,
                name: reg.name.trim_start_matches(name).trim_start_matches("_"),
                width: reg.width,
                access: reg.access,
                reset_value: reg.reset_value,
            })
            .collect::<Vec<_>>();

        vec![Peripheral {
            name,
            instances: Instances::Single(base_addr),
            registers,
        }]
    }
}

// example input: "7_REGISTER"
// example output: (7, "REGISTER")
fn extract_instance(s: &str) -> u8 {
    s.splitn(2, '_')
        .next()
        .unwrap_or("")
        .parse()
        .expect("split_instance: string doesn't start with a u8 integer")
}

fn find_common_prefix<'a>(
    rega: &'a str,
    regb: &'a str,
) -> (/* prefix */ &'a str, /* numbered */ bool) {
    let mut parts = rega.rsplitn(2, '_');
    let _suffix = parts.next();

    while let Some(mut maybe_prefix) = parts.next() {
        if regb.starts_with(maybe_prefix) {
            let mut numbered = false;

            while let Some(i) = maybe_prefix.char_indices().rev().next().and_then(|(i, c)| {
                if c.is_digit(10) {
                    Some(i)
                } else {
                    None
                }
            }) {
                maybe_prefix = &maybe_prefix[..i];
                numbered = true;
            }

            return (maybe_prefix, numbered);
        } else {
            parts = maybe_prefix.rsplitn(2, '_');
            let _suffix = parts.next();
        }
    }

    panic!("no common prefix between `{}` and `{}`", rega, regb);
}

#[cfg(test)]
mod tests {
    #[test]
    fn common_prefix() {
        assert_eq!(
            super::find_common_prefix("UART7_URXD", "UART7_UTXD"),
            ("UART", true)
        );

        assert_eq!(
            super::find_common_prefix("SNVS_HPLR", "SNVS_HPCOMR"),
            ("SNVS", false)
        );

        assert_eq!(
            super::find_common_prefix(
                "USB_ANALOG_USB1_VBUS_DETECT_STAT",
                "USB_ANALOG_USB1_CHRG_DETECT_STAT"
            ),
            ("USB_ANALOG_USB", true)
        );
    }

    #[test]
    fn extract_instance() {
        assert_eq!(super::extract_instance("7_URXD"), 7);
    }
}
