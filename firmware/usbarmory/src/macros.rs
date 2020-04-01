/// Prints message to the console IF, and only IF, the `Serial` is not currently "taken"
///
/// NOTE: this will likely result in missed messages if used in the context of
/// *asynchronous* interrupts
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        if let Some(serial) = $crate::serial::Serial::take() {
            serial.write_all($s.as_bytes());
            serial.release();
        }
    };

    ($s:expr, $($args:tt)*) => {
        if let Some(serial) = $crate::serial::Serial::take() {
            use core::fmt::Write as _;
            let _ = write!(&serial, $s, $($args)*); // never errors
            serial.release();
        }
    };
}

/// Just like `print!` but appends a newline at the end
#[macro_export]
macro_rules! println {
    ($s:expr) => {
        if let Some(serial) = $crate::serial::Serial::take() {
            serial.write_all(concat!($s, "\n").as_bytes());
            serial.release();
        }
    };

    ($s:expr, $($args:tt)*) => {
        if let Some(serial) = $crate::serial::Serial::take() {
            use core::fmt::Write as _;
            let _ = writeln!(&serial, $s, $($args)*); // never errors
            serial.release();
        }
    };
}

/// Flushes all the contents of the in-memory logger and then reboots the system
#[macro_export]
macro_rules! memlog_flush_and_reset {
    () => {
        $crate::memlog_flush_and_reset(file!(), line!())
    };
}

macro_rules! assume_unreachable {
    () => {
        if cfg!(debug_assertions) {
            unreachable!()
        } else {
            core::hint::unreachable_unchecked()
        }
    };
}
