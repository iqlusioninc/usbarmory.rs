use std::borrow::Cow;

use crate::{parse, Access};

#[derive(Debug, PartialEq)]
pub struct Register<'a> {
    /// Absolute address
    pub abs_addr: u32,
    pub description: Cow<'a, str>,
    pub name: &'a str,
    /// Width in bits
    pub width: u8,
    pub access: Access,
    pub reset_value: u32,
}

pub fn register_table(txt: &str) -> Vec<Register> {
    let mut prev = None;

    let mut registers = vec![];
    let mut lines = txt.lines().peekable();
    while let Some(line) = lines.next() {
        if let Some(register) = parse::line(line, prev, lines.peek().cloned()) {
            registers.push(register);
        }

        prev = Some(line);
    }

    registers
}

// the lines (table rows) we are interested in look like this:
//
// 12AB_56CD  Description (PERIPHERAL_REGISTER) 32 R/W 0000_8000h
//            ^^^^^^^^^^^                              ^^^^^^^^^^
// ^^^^^^^^^              ^^^^^^^^^^^^^^^^^^^^^    ^^^ reset value
// absolute address       identifier            ^^ access
//                                              width (in bits)
fn line<'a>(line: &'a str, prev: Option<&'a str>, next: Option<&'a str>) -> Option<Register<'a>> {
    let line = line.trim();
    let mut parts = line.splitn(2, ' ');
    let abs_addr = parse::abs_addr(parts.next()?)?;

    let rest = parts.next()?.trim();

    // the table row may be split in 3 lines with the description and identifier
    // being in the `prev` and `next` line but not in the current one (`line`).
    // Something like this:
    //
    // ```
    //            Description of                                   (..)
    // 12AB_56CD                                 32 R/W 0000_8000h
    //            register (PERIPHERAL_REGISTER)                   (..)
    // ```
    //
    // if the next part is `width` then we are in this case
    let mut parts = rest.splitn(2, ' ');
    let (description, name, width, rest) = if let Some(width) = parts.next().and_then(parse::width)
    {
        let prev = prev
            .expect("found split row; previous line is required")
            .trim();
        let next = next.expect("found split row; next line is required").trim();

        let (description, name) = if next.starts_with('(') {
            let next = next.trim_start_matches('(');

            let mut parts = prev.rsplitn(2, ' ');
            let last = parts.next().unwrap_or("");
            let description = if last.starts_with(|c| char::is_digit(c, 10)) && last.contains(".") {
                // section information at the end; discard it
                parts.next().expect("UNREACHABLE").trim()
            } else {
                prev
            };

            let mut parts = next.splitn(2, ')');
            let name = parts.next().expect("UNREACHABLE");

            (Cow::Borrowed(description), name)
        } else {
            unimplemented!()
        };

        (description, name, width, parts.next()?.trim())
    } else {
        let mut parts = rest.splitn(2, '(');
        let description = parts.next()?.trim();
        let rest = parts.next()?;

        let mut parts = rest.splitn(2, ')');
        let name = parts.next()?;
        if name.contains(' ') || !name.contains('_') {
            return None;
        }

        let rest = parts.next()?.trim();
        let mut parts = rest.splitn(2, ' ');
        let width = parse::width(parts.next()?)?;

        (
            Cow::Borrowed(description),
            name,
            width,
            parts.next()?.trim(),
        )
    };

    let mut parts = rest.splitn(2, ' ');
    let access = parse::access(parts.next()?)?;

    let rest = parts.next()?.trim();
    let mut parts = rest.splitn(2, ' ');
    let reset_value = parse::reset_value(parts.next()?)?;

    Some(Register {
        abs_addr,
        description,
        name,
        width,
        access,
        reset_value,
    })
}

// parses an absolute address
//
// `C` or `210_34DC`
fn abs_addr(txt: &str) -> Option<u32> {
    let mut parts = txt.splitn(2, '_');
    let halfword = u16::from_str_radix(parts.next()?, 16).ok()?;
    if let Some(rest) = parts.next() {
        let upper = halfword;
        let lower = u16::from_str_radix(rest, 16).ok()?;
        Some((u32::from(upper) << 16) | u32::from(lower))
    } else {
        Some(u32::from(halfword))
    }
}

// width must be a multiple of 8
fn width(txt: &str) -> Option<u8> {
    match txt {
        "8" | "16" | "32" => txt.parse().ok(),
        _ => None,
    }
}

fn access(txt: &str) -> Option<Access> {
    Some(match txt {
        "R" => Access::Read,
        "R/W" => Access::ReadWrite,
        "W" => Access::Write,
        "w1c" => Access::WriteOneToClear,
        _ => return None,
    })
}

// `12AB_34CDh`
fn reset_value(mut txt: &str) -> Option<u32> {
    if !txt.ends_with("h") {
        return None;
    }

    txt = &txt[..txt.len() - 1];
    let mut parts = txt.splitn(2, '_');

    let higher = u16::from_str_radix(parts.next()?, 16).ok()?;
    let lower = u16::from_str_radix(parts.next()?, 16).ok()?;

    Some(u32::from(higher) << 16 | u32::from(lower))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{parse::Register, Access};

    #[test]
    fn sanity() {
        let registers = crate::parse::register_table("  201_8000   UART Receiver Register (UART7_URXD)                        32       R       0000_0000h
");

        assert_eq!(registers.len(), 1);
        assert_eq!(
            registers[0],
            Register {
                abs_addr: 0x201_8000,
                description: Cow::Borrowed("UART Receiver Register"),
                name: "UART7_URXD",
                width: 32,
                access: Access::Read,
                reset_value: 0,
            }
        );
    }

    #[test]
    fn split() {
        let registers = crate::parse::register_table("\
             SNVS_HP Real-Time Counter MSB Register
 20C_C024                                                         32      R/W      0000_0000h    46.7.5/2932
             (SNVS_HPRTCMR)");

        assert_eq!(registers.len(), 1);
        assert_eq!(
            registers[0],
            Register {
                abs_addr: 0x20C_C024,
                description: Cow::Borrowed("SNVS_HP Real-Time Counter MSB Register"),
                name: "SNVS_HPRTCMR",
                width: 32,
                access: Access::ReadWrite,
                reset_value: 0,
            }
        );

        let registers = crate::parse::register_table(
"            SNVS_LP Secure Monotonic Counter MSB Register                                        46.7.12/
 20C_C05C                                                         32      R/W      0000_0000h
             (SNVS_LPSMCMR)                                                                        2942");

        assert_eq!(registers.len(), 1);
        assert_eq!(
            registers[0],
            Register {
                abs_addr: 0x20C_C05C,
                description: Cow::Borrowed("SNVS_LP Secure Monotonic Counter MSB Register"),
                name: "SNVS_LPSMCMR",
                width: 32,
                access: Access::ReadWrite,
                reset_value: 0,
            }
        );
    }
}
