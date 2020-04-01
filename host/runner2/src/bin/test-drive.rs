use core::str;

use runner2::sdp::Sdp;

fn main() {
    println!("> attempting to claim USB device (this should complete in less than 4 seconds)");
    let sdp = Sdp::open().expect("failed to open USB device. Is the Armory in 'eMMC' boot mode and connected to your PC? Does the USB device `15a2:0080` show under `lsusb`?");
    println!("> performing an HID exchange with the USB device to read part of the ROM");
    let mut bytes = vec![];
    let word = sdp.read_memory(0x84).expect("couldn't read device memory");
    bytes.extend_from_slice(&word.to_le_bytes());
    println!("> first HID exchanged succeeded; performing some more");
    let mut addr = 0x88;
    let end = 0xb0;
    while addr < end {
        let word = sdp.read_memory(addr).expect("couldn't read device memory");
        bytes.extend_from_slice(&word.to_le_bytes());
        addr += 4;
    }
    println!("> printing the data we just read. You should see a string");
    if let Ok(s) = str::from_utf8(&bytes) {
        println!("{:?}", s);
    } else {
        println!("{:?}", bytes);
    }

    println!("> releasing USB device (this should NOT take several seconds)");
    drop(sdp);
    println!("> all done!");
}
