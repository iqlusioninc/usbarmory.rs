//! Parses a program image and prints its contents using the `fmt::Debug` formatter

use std::{env, fs};

use anyhow::bail;
use image::read::Image;

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let bytes = fs::read(&args[0])?;
    let image = Image::parse(&bytes)?;
    println!("{:#?}", image);

    Ok(())
}
