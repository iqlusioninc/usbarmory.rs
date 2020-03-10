#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    emmc::eMMC,
    memlog, memlog_flush_and_reset,
    storage::{Block, MbrDevice, ManagedBlockDevice},
    fs::{File, LittleFs},
};
use littlefs2::io;

static TESTSTR: &[u8] = b"Hello File!";

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let emmc = eMMC::take().expect("eMMC");
    for _ in 0..4 { usbarmory::memlog_try_flush(); }

    let mut mbr = MbrDevice::open(emmc).unwrap();
    let mut main_part = mbr.partition(1).unwrap();
    for _ in 0..4 { usbarmory::memlog_try_flush(); }

    // Corrupt the file system by overwriting the first 2 blocks, which store reduntant copies of
    // the superblock.
    let block = Block::zeroed();
    main_part.write(&block, 0).unwrap();
    main_part.write(&block, 1).unwrap();

    if let Err(littlefs2::io::Error::Corruption) = LittleFs::mount_and_then(&mut main_part, |_| Ok(())) {
        memlog!("Formatting disk");
        LittleFs::format(&mut main_part).unwrap();
    }

    for _ in 0..4 { usbarmory::memlog_try_flush(); }
    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        memlog!("{} bytes free", fs.available_space()?);

        File::create_and_then(fs, "/testfile-a", |file| -> io::Result<()> {
            file.write(TESTSTR)?;
            file.flush()?;
            assert_eq!(file.len()?, TESTSTR.len());
            Ok(())
        })?;

        for ent in fs.read_dir("/")? {
            let ent = ent?;
            memlog!("{:?}", ent.file_type());
            usbarmory::memlog_try_flush();
        }

        Ok(())
    }).unwrap();

    LittleFs::mount_and_then(&mut main_part, |fs| -> io::Result<()> {
        File::open_and_then(fs, "/testfile-a", |file| -> io::Result<()> {
            assert_eq!(file.len()?, TESTSTR.len());

            let mut buf = [0; 32];
            let bytes = file.read(&mut buf)?;
            if !buf.starts_with(TESTSTR) {
                panic!("{} bytes: {:x?}", bytes, buf);
            }
            Ok(())
        })?;

        Ok(())
    }).unwrap();

    // then reset the board to return to the u-boot console
    memlog_flush_and_reset!();
}
