//! Image reader

use anyhow::format_err;

use crate::{BootData, Dcd, Ivt};

/// Parsed program image (does not contain application data)
#[derive(Debug)]
pub struct Image<'a> {
    /// Image Vector Table
    pub ivt: Ivt,

    /// Boot data
    pub boot_data: BootData,

    /// Optional DCD
    pub dcd: Option<Dcd<'a>>,
}

impl<'a> Image<'a> {
    /// Parses a program image
    pub fn parse(input: &'a [u8]) -> Result<Self, anyhow::Error> {
        let ivt = Ivt::parse(input)
            .map_err(|_| format_err!("error parsing the IVT"))?
            .1;
        let boot_data_offset = ivt.boot.checked_sub(ivt.self_).expect("TODO") as usize;
        let boot_data = BootData::parse(
            input
                .get(boot_data_offset..boot_data_offset + usize::from(BootData::SIZE))
                .expect("TODO"),
        )
        .map_err(|_| format_err!("error parsing Boot Data"))?
        .1;
        let dcd = if ivt.dcd != 0 {
            let dcd_offset = ivt.dcd.checked_sub(ivt.self_).expect("TODO") as usize;
            Some(
                Dcd::parse(input.get(dcd_offset..).expect("TODO"))
                    .map_err(|_| format_err!("error parsing DCD"))?
                    .1,
            )
        } else {
            None
        };

        Ok(Image {
            ivt,
            boot_data,
            dcd,
        })
    }
}
