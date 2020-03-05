//! Data Co-Processor (DCP)
//!
//! The DCP supports the following operations
//!
//! - AES-128, in ECB and CBC modes
//! - SHA-256
//! - SHA-1
//!
//! Only AES-128 in ECB mode is currently exposed

// References: section 13 of MX28RM

use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{self, AtomicBool, Ordering},
    time::Duration,
};

use arrayref::array_ref;
use block_cipher_trait::{
    generic_array::{typenum::consts, GenericArray},
    BlockCipher,
};
use pac::hw_dcp::HW_DCP;

use crate::{memlog, memlog_flush_and_reset, util};

fn default_timeout() -> Duration {
    Duration::from_millis(100)
}

/// Data Co-Processor
pub struct Aes128 {
    key: KeySelect,
    // this struct implicitly owns a shared reference to `HW_DCP` ..
    _not_send_or_sync: PhantomData<*mut ()>,
}

// .. however it accesses different registers so instances are OK to `Send`
unsafe impl Send for Aes128 {}

const STAT_ERROR_MASK: u32 = (0xff << 16) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2);

impl BlockCipher for Aes128 {
    type KeySize = consts::U16;
    type BlockSize = consts::U16;
    type ParBlocks = consts::U1;

    fn new(key: &GenericArray<u8, consts::U16>) -> Self {
        // TODO support up to 4 live ciphers (active channels)
        assert!(init(), "no more ciphers can't be created");

        // start from the highest numbered channel to reduce the size of the `Context` struc
        let channel = 3;

        HW_DCP::borrow_unchecked(|dcp| {
            // install key in the write-only register
            // NOTE to support more than one cipher this would need a mutex (e.g. spinlock) to do a
            // exclusive write to these registers
            dcp.KEY.write(channel << 6);
            // XXX double check endianness
            dcp.KEYDATA
                .write(u32::from_le_bytes(*array_ref!(key, 0, 4)));
            dcp.KEYDATA
                .write(u32::from_le_bytes(*array_ref!(key, 4, 4)));
            dcp.KEYDATA
                .write(u32::from_le_bytes(*array_ref!(key, 8, 4)));
            dcp.KEYDATA
                .write(u32::from_le_bytes(*array_ref!(key, 12, 4)));

            // enable channel #3
            // NOTE single instruction write to a stateless register
            dcp.CHANNELCTRL_SET.write(1 << channel);
        });

        Aes128 {
            key: KeySelect::Key0,
            _not_send_or_sync: PhantomData,
        }
    }

    fn decrypt_block(&self, block: &mut GenericArray<u8, consts::U16>) {
        self.xcrypt(block, false)
    }

    fn encrypt_block(&self, block: &mut GenericArray<u8, consts::U16>) {
        self.xcrypt(block, true)
    }
}

impl Aes128 {
    /// Creates a cipher that uses the unreadable OTP key
    ///
    /// **WARNING!** This routine does NOT check if the OTP was set to an all-zeros value
    pub fn new_otp() -> Self {
        Self::new_hardware(true)
    }

    /// Creates a cipher that uses the unreadable UNIQUE key
    ///
    /// This UNIQUE key is generated from the OTP key and a device-specific 64-bit value
    pub fn new_unique() -> Self {
        Self::new_hardware(false)
    }

    // Creates a cipher that uses one of the two available hardware keys
    fn new_hardware(otp: bool) -> Self {
        // TODO support up to 4 live ciphers (active channels)
        assert!(init(), "no more ciphers can't be created");

        // start from the highest numbered channel to reduce the size of the `Context` struc
        let channel = 3;

        HW_DCP::borrow_unchecked(|dcp| {
            const STAT_OTP_KEY_READY: u32 = 1 << 28;

            // check that the OTP is ready to use ("has been released")
            let mut stat = 0;
            let is_ready = || {
                stat = dcp.STAT.read();
                stat & STAT_OTP_KEY_READY != 0
            };
            if util::wait_for_or_timeout(is_ready, default_timeout()).is_err() {
                memlog!("OTP key not released within timeout (STAT={:#010x})", stat);
                memlog_flush_and_reset!()
            }

            // enable channel #3
            // NOTE single instruction write to a stateless register
            dcp.CHANNELCTRL_SET.write(1 << channel);
        });

        Aes128 {
            key: if otp {
                KeySelect::OtpKey
            } else {
                KeySelect::UniqueKey
            },
            _not_send_or_sync: PhantomData,
        }
    }

    fn xcrypt(&self, block: &mut GenericArray<u8, consts::U16>, encrypt: bool) {
        let cmd = Cmd::new();

        cmd.control0.set(
            *Control0::new()
                .decr_semaphore(true)
                .enable_cipher(true)
                .cipher_encrypt(encrypt)
                .otp_key(self.key == KeySelect::OtpKey),
        );

        cmd.control1.set(
            *Control1::new()
                .cipher_select(CipherSelect::Aes128)
                .cipher_mode(CipherMode::Ecb)
                .key_select(self.key),
        );

        cmd.src_buffer_addr.set(Some(NonNull::from(&block[0])));
        cmd.dest_buffer_addr.set(Some(NonNull::from(&mut block[0])));
        cmd.buffer_size.set(block.len());

        HW_DCP::borrow_unchecked(|dcp| {
            // TODO generalize over channel index
            dcp.CH3CMDPTR.write(&cmd as *const Cmd as u32);

            // the write below transfers ownership of `block` and `cmd` to the crypto engine; this
            // fence drives all pending writes to `block` & `cmd` to completion
            atomic::fence(Ordering::Release);

            // start encryption
            dcp.CH3SEMA.write(1);

            // wait for channel to signal it's done -- the trait interface requires this operation
            // to be blocking
            let mut stat = 0;
            let mut status = 0;
            let is_done_or_error = || {
                // NOTE(read_volatile) we want this statement to always perform a load
                // instruction rather than read memory once and cache the value in a (CPU)
                // register
                status = unsafe { cmd.status.get().read_volatile() };
                stat = dcp.CH3STAT.read();

                // done or error
                status & 1 != 0 || stat & STAT_ERROR_MASK != 0
            };
            if util::wait_for_or_timeout(is_done_or_error, default_timeout()).is_err() {
                memlog!("encryption timeout (STAT={:#010x})", stat);
                memlog_flush_and_reset!()
            }

            // the crypto engine is done with the block and has transferred ownership back to us
            atomic::fence(Ordering::Acquire);

            // the interface won't let us report the error so just abort the program
            if stat & STAT_ERROR_MASK != 0 {
                memlog!("error (STAT={:#010x}, Cmd.status={:#010x})", stat, status);
                memlog_flush_and_reset!()
            }
        })
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

// Big enough for one channel (channel #3)
const CTXT_SZ: usize = 52;

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
        // hash_select = (HashSelect, 0xf, 16),
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

#[cfg(unused)]
enum HashSelect {
    Sha1 = 0,
    Crc32 = 1,
    Sha256 = 2,
}
