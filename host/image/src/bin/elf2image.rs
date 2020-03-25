//! Converts an ELF into an image suitable for flashing into the eMMC

use std::{
    env,
    fs::{self, File},
    path::Path,
};

use anyhow::{bail, format_err};
use image::write::Image;
use xmas_elf::ElfFile;

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let path = Path::new(&args[0]);
    let stem =
        Path::new(path.file_stem().ok_or_else(|| {
            format_err!("unable to determine the file stem of {}", path.display())
        })?);
    let bytes = fs::read(path)?;
    let elf = ElfFile::new(&bytes).map_err(|s| format_err!("{}", s))?;
    // we always need to initialize the DDR when cold booting from the eMMC
    let skip_dcd = false;
    let image = Image::from_elf(&elf, skip_dcd)?;
    image.write(&mut File::create(stem.with_extension("bin"))?)?;

    Ok(())
}
