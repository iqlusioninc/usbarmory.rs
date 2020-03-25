//! Image writer

use core::{cmp, ops::Range};
use std::io::{self, Write};

use anyhow::format_err;
use xmas_elf::{
    program::{SegmentData, Type},
    sections::SectionData,
    symbol_table::Entry as _,
    ElfFile,
};

use crate::{
    dcd::{self, AddressValue},
    ivt, BootData, Ivt,
};

const K: u32 = 1024;
const M: u32 = 1024 * K;
const OCRAM_START: u32 = 0x00900000;
const DRAM_START: u32 = 0x80000000;
const OCRAM_SIZE: u32 = 128 * K;
const DRAM_SIZE: u32 = 512 * M;

const OCRAM: Range<u32> = OCRAM_START..OCRAM_START + OCRAM_SIZE;
const DRAM: Range<u32> = DRAM_START..DRAM_START + DRAM_SIZE;

/// Image to write
pub struct Image {
    /// Application data (.text, .rodata, etc.)
    pub app: Vec<u8>,

    /// Optional DCD
    pub dcd: Option<Dcd>,

    /// The address of the entry point
    // NOTE currently hardcoded to point to the "_start" symbol
    pub entry: u32,
}

/// Write commands that initialize the external DDR RAM
fn init_ddr() -> Vec<AddressValue> {
    macro_rules! write {
        ($writes:expr, $register:ty, $value:expr) => {
            $writes.push(AddressValue {
                address: <$register>::address() as u32,
                value: $value,
            });
        };
    }

    use pac::{iomuxc, mmdc};

    let mut writes = vec![];

    /* IO configuration */
    // select DDR3 mode for DRAM pins
    write!(writes, iomuxc::SW_PAD_CTL_GRP_DDR_TYPE, 0x000c_0000);

    // disable the pull-up of the DRAM pins
    write!(writes, iomuxc::SW_PAD_CTL_GRP_DDRPKE, 0);

    // disable pull-ups
    write!(writes, iomuxc::SW_PAD_CTL_PAD_DRAM_RESET, 0x8_0030);
    write!(writes, iomuxc::SW_PAD_CTL_PAD_DRAM_SDBA2, 0);
    write!(writes, iomuxc::SW_PAD_CTL_PAD_DRAM_ODT0, 0x30);
    write!(writes, iomuxc::SW_PAD_CTL_PAD_DRAM_ODT1, 0x30);
    // FIXME(txt2rust) register `SW_PAD_CTL_PAD_DRAM_SDQS0_P` is missing
    writes.push(AddressValue {
        address: 0x020e0280,
        value: 0x30,
    });
    // FIXME(txt2rust) register `SW_PAD_CTL_PAD_DRAM_SDQS1_P` is missing
    writes.push(AddressValue {
        address: 0x020e0284,
        value: 0x30,
    });

    // differential input mode
    write!(writes, iomuxc::SW_PAD_CTL_GRP_DDRMODE_CTL, 0x0002_0000);
    write!(writes, iomuxc::SW_PAD_CTL_GRP_DDRMODE, 0x0002_0000);

    /* DDR RAM configuration */
    // section 33.4.2 of 6ULRM

    // configuration request
    write!(writes, mmdc::MDSCR, 0x8000);

    // set timing parameters
    write!(writes, mmdc::MDCFG0, 0x676b_52f3);
    write!(writes, mmdc::MDCFG1, 0xb66d_0b63);
    write!(writes, mmdc::MDCFG2, 0x01ff_00db);
    write!(writes, mmdc::MDOTC, 0x1b33_3030);

    // set DDR type (DDR3)
    write!(writes, mmdc::MDMISC, 0x0020_1740);

    // set delay parameters
    write!(writes, mmdc::MDRWD, 0x26d2);

    // size = 512 MB
    write!(writes, mmdc::MDASP, 0x4f);

    // out of reset delay
    write!(writes, mmdc::MDOR, 0x006b_1023);

    // configure density
    write!(writes, mmdc::MDCTL, 0x8418_0000);

    // ZQ
    write!(writes, mmdc::MPZQHWCTRL, 0xa139_0003);

    // delay configuration
    write!(writes, mmdc::MPWLDECTRL0, 0x000d_000f);
    write!(writes, mmdc::MPWLDECTRL1, 0x0010_0010);

    // DQS
    write!(writes, mmdc::MPDGCTRL0, 0x415c_0160);

    // delay-lines
    write!(writes, mmdc::MPRDDLCTL, 0x4040_3c42);
    write!(writes, mmdc::MPWRDLCTL, 0x4040_2c26);

    // DQ
    write!(writes, mmdc::MPRDDQBY0DL, 0x3333_3333);
    write!(writes, mmdc::MPRDDQBY1DL, 0x3333_3333);
    write!(writes, mmdc::MPWRDQBY0DL, 0xf333_3333);
    write!(writes, mmdc::MPWRDQBY1DL, 0xf333_3333);

    // duty cycle
    write!(writes, mmdc::MPDCCR, 0x0092_1012);

    // force measurements on delay-lines
    write!(writes, mmdc::MPMUR0, 0x0800);

    write!(writes, mmdc::MDSCR, 0x0200_8032);
    write!(writes, mmdc::MDSCR, 0x8033);
    write!(writes, mmdc::MDSCR, 0x0004_8031);
    write!(writes, mmdc::MDSCR, 0x1520_8030);

    // ZQ calibration
    write!(writes, mmdc::MDSCR, 0x0400_8040);

    // power-down configuration
    write!(writes, mmdc::MDPDC, 0x0002_556d);
    write!(writes, mmdc::MAPSR, 0x0001_1006);

    // periodic refresh scheme
    write!(writes, mmdc::MDREF, 0x0800);

    // enable resistor
    write!(writes, mmdc::MPODTCTRL, 0x0227);

    // configuration done
    write!(writes, mmdc::MDSCR, 0);

    writes
}

// space reserved for the IVT, DCD and Boot Data
// NOTE if you change this you'll need to update `usbarmory-rt/link.x`
const RESERVED: u32 = 1024;
// space between the start of DRAM and the IVT (this is required to boot from the eMMC/uSD)
const PADDING: u32 = 1024;

impl Image {
    /// Creates image data from an ELF file
    pub fn from_elf(elf: &ElfFile<'_>, skip_dcd: bool) -> Result<Image, anyhow::Error> {
        // sections to include in the program image, iff they are in DRAM
        const SECTIONS: &[&str] = &[".text", ".rodata", ".start"];

        // do a first pass to determine the size of the `app` buffer
        let mut start = None;
        let mut end = None;
        for sh in elf.section_iter() {
            if let Ok(name) = sh.get_name(elf) {
                let sect_start = sh.address() as u32;

                if SECTIONS.contains(&name) && DRAM.contains(&sect_start) {
                    let size = sh.size() as u32;
                    let sect_end = sect_start + size;

                    if let Some(start) = start.as_mut() {
                        *start = cmp::min(*start, sect_start);
                    } else {
                        start = Some(sect_start);
                    }

                    if let Some(end) = end.as_mut() {
                        *end = cmp::max(*end, sect_end);
                    } else {
                        end = Some(sect_end);
                    }
                }
            }
        }

        for ph in elf.program_iter() {
            if ph.get_type() == Ok(Type::Load) {
                let phys_addr = ph.physical_addr() as u32;
                let virt_addr = ph.virtual_addr() as u32;

                // these are sections that need to be copied from DRAM To OCRAM
                if OCRAM.contains(&virt_addr) && DRAM.contains(&phys_addr) {
                    let sect_start = phys_addr;
                    let size = ph.mem_size() as u32;
                    let sect_end = sect_start + size;

                    if let Some(start) = start.as_mut() {
                        *start = cmp::min(*start, sect_start);
                    } else {
                        start = Some(sect_start);
                    }

                    if let Some(end) = end.as_mut() {
                        *end = cmp::max(*end, sect_end);
                    } else {
                        end = Some(sect_end);
                    }
                }
            }
        }

        let (start, end) = (start.expect("TODO"), end.expect("TODO"));

        // do a second pass to fill the `app` buffer
        let mut app = vec![0u8; end as usize - start as usize];
        for sh in elf.section_iter() {
            if let Ok(name) = sh.get_name(elf) {
                let addr = sh.address() as u32;

                if SECTIONS.contains(&name) && DRAM.contains(&addr) {
                    let off = (addr - start) as usize;
                    let size = sh.size() as usize;
                    let data = sh.raw_data(elf);

                    app[off..(off + size)].copy_from_slice(data)
                }
            }
        }

        for ph in elf.program_iter() {
            if ph.get_type() == Ok(Type::Load) {
                let phys_addr = ph.physical_addr() as u32;
                let virt_addr = ph.virtual_addr() as u32;

                // this is the .data section
                if OCRAM.contains(&virt_addr) && DRAM.contains(&phys_addr) {
                    let data =
                        if let SegmentData::Undefined(bytes) = ph.get_data(&elf).expect("TODO") {
                            bytes
                        } else {
                            panic!("TODO")
                        };
                    let size = ph.mem_size() as usize;
                    let off = (phys_addr - start) as usize;

                    app[off..(off + size)].copy_from_slice(data);
                }
            }
        }

        // FIXME this shouldn't be hardcoded but instead should fetch the entry point from the ELF
        // metadata (see "Entry point address" in the output of `readelf -h $elf`)
        // look for the entry point named "_start"
        let mut start = None;
        let symtab = elf.find_section_by_name(".symtab").ok_or_else(|| {
            format_err!(
                "`.symtab`
section not found in ELF file"
            )
        })?;
        let data = symtab.get_data(&elf).map_err(|s| format_err!("{}", s))?;
        if let SectionData::SymbolTable32(entries) = data {
            for entry in entries {
                if entry.get_name(&elf) == Ok("_start") {
                    start = Some(entry.value() as u32);
                }
            }
        }

        Ok(Image {
            dcd: if skip_dcd {
                None
            } else {
                Some(Dcd { writes: init_ddr() })
            },
            app,
            entry: start.ok_or_else(|| format_err!("symbol `_start` was not found"))?,
        })
    }

    /// Writes the program image
    pub fn write(mut self, w: &mut impl Write) -> io::Result<()> {
        let ivt = Ivt {
            header: ivt::Header::default(),
            self_: DRAM_START + PADDING,
            boot: DRAM_START + PADDING + u32::from(Ivt::SIZE),
            dcd: if self.dcd.is_some() {
                DRAM_START + PADDING + u32::from(Ivt::SIZE + BootData::SIZE)
            } else {
                0
            },
            // unused at the moment but would be placed after the DCD
            csf: 0,
            entry: self.entry,
        };
        let boot_data = BootData {
            len: PADDING + RESERVED + self.app.len() as u32,
            plugin: 0,
            start: DRAM_START,
        };
        let mut pos = 0;
        ivt.write(w)?;
        pos += u32::from(Ivt::SIZE);
        boot_data.write(w)?;
        pos += u32::from(BootData::SIZE);
        if let Some(dcd) = self.dcd.take() {
            pos += u32::from(dcd.size());
            dcd.write(w)?;
        }
        w.write_all(&vec![0; RESERVED.checked_sub(pos).expect("TODO") as usize])?;
        w.write_all(&self.app)?;
        Ok(())
    }
}

/// DCD
pub struct Dcd {
    /// Write Data command
    pub writes: Vec<AddressValue>,
}

impl Dcd {
    fn write(self, w: &mut impl Write) -> io::Result<()> {
        let mut header = dcd::Header::default();
        header.length = self.size();

        header.write(w)?;
        let command = dcd::WriteDataCommand::writes(self.writes);
        command.write(w)?;
        Ok(())
    }

    fn size(&self) -> u16 {
        u16::from(dcd::Header::SIZE + dcd::WriteDataCommandHeader::SIZE)
            + 8 * self.writes.len() as u16
    }
}

impl dcd::Header {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_all(&[self.tag])?;
        w.write_all(&self.length.to_be_bytes())?;
        w.write_all(&[self.version])?;
        Ok(())
    }
}

impl dcd::WriteDataCommand {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        self.header.write(w)?;
        assert_eq!(self.header.parameter.bytes, dcd::Bytes::B4, "unimplemented");
        for av in &self.address_value {
            w.write_all(&av.address.to_be_bytes())?;
            w.write_all(&av.value.to_be_bytes())?;
        }
        Ok(())
    }
}

impl dcd::WriteDataCommandHeader {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_all(&[self.tag])?;
        w.write_all(&self.length.to_be_bytes())?;
        let mut byte = self.parameter.bytes as u8;
        byte |= (self.parameter.flags as u8) << 3;
        w.write_all(&[byte])?;
        Ok(())
    }
}

impl ivt::Header {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_all(&[self.tag])?;
        w.write_all(&self.length.to_be_bytes())?;
        w.write_all(&[self.version])?;
        Ok(())
    }
}

impl Ivt {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        self.header.write(w)?;
        w.write_all(&self.entry.to_le_bytes())?;
        w.write_all(&[0; 4])?; // reserved1
        w.write_all(&self.dcd.to_le_bytes())?;
        w.write_all(&self.boot.to_le_bytes())?;
        w.write_all(&self.self_.to_le_bytes())?;
        w.write_all(&self.csf.to_le_bytes())?;
        w.write_all(&[0; 4])?; // reserved2
        Ok(())
    }
}

impl BootData {
    fn write(&self, w: &mut impl Write) -> io::Result<()> {
        w.write_all(&self.start.to_le_bytes())?;
        w.write_all(&self.len.to_le_bytes())?;
        w.write_all(&self.plugin.to_le_bytes())?;
        Ok(())
    }
}
