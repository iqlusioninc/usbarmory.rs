//! Data Co-Processor (DCP)
//!
//! The DCP supports the following operations
//!
//! - AES-128, in ECB and CBC modes
//! - SHA-256
//! - SHA-1
//!
//! Only SHA-256 & AES-128 in ECB mode are currently exposed
//!
//! **NOTE** The DCP is only available on the i.MX6UL**Z** chip

// References: section 13 of MX28RM

use core::{
    cell::{Cell, UnsafeCell},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use pac::HW_DCP;

pub use aes128::Aes128;
pub use sha256::Sha256;

mod aes128;
mod sha256;

fn default_timeout() -> Duration {
    Duration::from_millis(100)
}

const STAT_ERROR_MASK: u32 = (0xff << 16) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2);

// start from the highest numbered channel to reduce the size of the `Context` struc
const AES128_HW_CHANNEL: u8 = 3;
const SHA256_CHANNEL: u8 = 2;
const AES128_RAM_CHANNEL: u8 = 1;

// Big enough for two channels (channels #1, #2 & #3) -- see table 13-1
const CTXT_SZ: usize = 52 * 3;

// word-aligned for performance
#[repr(align(4))]
struct Context {
    _bytes: [u8; CTXT_SZ],
}

impl Context {
    const fn empty() -> Self {
        Self {
            _bytes: [0; CTXT_SZ],
        }
    }
}

fn init() -> bool {
    static INITIALIZED: AtomicBool = AtomicBool::new(false);

    if INITIALIZED
        .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        let dcp = HW_DCP::take().expect("DCP peripheral has already been taken");

        assert_eq!(
            dcp.VERSION.read(),
            (1 << 25) | (1 << 16),
            "unsupported DCP version"
        );

        let supports_aes128 = 1;
        let supports_sha256 = 1 << 18;
        let capability = dcp.CAPABILITY1.read();
        assert!(
            capability & supports_aes128 != 0,
            "this DCP doesn't support AES128"
        );

        assert!(
            capability & supports_sha256 != 0,
            "this DCP doesn't support SHA256"
        );

        // NOTE(static mut) this code runs at most once; CTXT will never be aliased
        static mut CTXT: UnsafeCell<Context> = UnsafeCell::new(Context::empty());

        // install context
        dcp.CONTEXT.write(unsafe { CTXT.get() as u32 });

        true
    } else {
        // already initialized
        false
    }
}

// MX28RM section 13.2.6.4
// NOTE instances of this will be shared with the hardware (the hardware can modify its fields) so
// one should never create an exclusive reference (`&mut`) to this struct
#[repr(C)]
struct Cmd {
    next_cmd_addr: Cell<Option<NonNull<Cmd>>>,
    control0: Cell<Control0>,
    control1: Cell<Control1>,
    src_buffer_addr: Cell<Option<NonNull<u8>>>,
    dest_buffer_addr: Cell<Option<NonNull<u8>>>,
    buffer_size: Cell<usize>,
    payload_pointer: Cell<Option<NonNull<u8>>>,
    // NOTE(UnsafeCell) will be modified by the hardware
    status: UnsafeCell<u32>,
}

impl Cmd {
    const fn new() -> Self {
        Self {
            next_cmd_addr: Cell::new(None),
            control0: Cell::new(Control0::new()),
            control1: Cell::new(Control1::new()),
            src_buffer_addr: Cell::new(None),
            dest_buffer_addr: Cell::new(None),
            buffer_size: Cell::new(0),
            payload_pointer: Cell::new(None),
            status: UnsafeCell::new(0),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct Control0 {
    bits: u32,
}

macro_rules! setters0 {
	  ($($setter:ident = $offset:expr,)+) => {
        $(
            #[allow(dead_code)]
            fn $setter(&mut self, on: bool) -> &mut Self {
                const MASK: u32 = 1 << $offset;

                if on {
                    self.bits |= MASK;
                } else {
                    self.bits &= !MASK;
                }
                self
            }
        )+
	  };
}

impl Control0 {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    #[allow(dead_code)]
    fn tag(&mut self, tag: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u32 = 0xff << OFFSET;
        self.bits &= !MASK;
        self.bits |= u32::from(tag) << OFFSET;
        self
    }

    setters0!(
        interrupt = 0,
        decr_semaphore = 1,
        chain = 2,
        chain_contiguous = 3,
        enable_memcpy = 4,
        enable_cipher = 5,
        enable_hash = 6,
        enable_blit = 7,
        cipher_encrypt = 8,
        cipher_init = 9,
        otp_key = 10,
        payload_key = 11,
        hash_init = 12,
        hash_term = 13,
        check_hash = 14,
        hash_output = 15,
        constant_fill = 16,
        key_byteswap = 18,
        key_wordswap = 19,
        input_byteswap = 20,
        input_wordswap = 21,
        output_byteswap = 22,
        output_wordswap = 23,
    );
}

macro_rules! setters1 {
	  ($($setter:ident = ($enum_:ident, $mask:expr, $offset:expr),)+) => {
        $(
            #[allow(dead_code)]
            fn $setter(&mut self, val: $enum_) -> &mut Self {
                const OFFSET: u8 = $offset;
                const MASK: u32 = $mask << $offset;

                self.bits &= !MASK;
                self.bits |= (val as u32) << OFFSET;
                self
            }
        )+
	  };
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct Control1 {
    bits: u32,
}

impl Control1 {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    setters1!(
        cipher_select = (CipherSelect, 0xf, 0),
        cipher_mode = (CipherMode, 0xf, 4),
        key_select = (KeySelect, 0xff, 8),
        hash_select = (HashSelect, 0xf, 16),
    );
}

enum CipherSelect {
    Aes128 = 0,
}

#[allow(dead_code)]
enum CipherMode {
    Ecb = 0,
    Cbc = 1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum KeySelect {
    Key0 = 0,
    // Key1 = 1,
    // Key2 = 2,
    // Key3 = 3,
    UniqueKey = 0xfe,
    OtpKey = 0xff,
}

enum HashSelect {
    // Sha1 = 0,
    // Crc32 = 1,
    Sha256 = 2,
}
