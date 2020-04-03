use std::{env, fs};

use anyhow::bail;
use arrayref::array_ref;
use image::read::Image;
use usd::Usd;

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let mut bytes = fs::read(&args[0])?;
    let image = Image::parse(&bytes)?;
    eprintln!("{:#?}", image.ivt);
    let address = image.ivt.self_;

    let mut usd = Usd::open()?;
    usd.set_verbose(true);

    let mut clear_dcd = false;
    if let Some(dcd) = image.dcd.as_ref() {
        clear_dcd = true;
        let dcd_ptr = u32::from_le_bytes(*array_ref!(bytes, 12, 4));
        usd.dcd_write(usd::OCRAM_FREE_ADDRESS, dcd.as_bytes())?;
        println!("clearing DCD pointer ({:#010x})", dcd_ptr);
    }

    // make it as if the image contained no DCD -- we don't want the device to run it twice
    if clear_dcd {
        bytes[12..16].iter_mut().for_each(|b| *b = 0);
    }

    usd.write_file(address, &bytes)?;
    usd.jump_address(address)?;

    eprintln!("DONE");

    Ok(())
}
