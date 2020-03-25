use core::{
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{self, AtomicBool, Ordering},
};

use arrayref::array_ref;
use block_cipher_trait::{
    generic_array::{typenum::consts, GenericArray},
    BlockCipher,
};
use pac::HW_DCP;

use crate::{
    dcp::{
        self, CipherMode, CipherSelect, Cmd, Control0, Control1, KeySelect, AES128_HW_CHANNEL,
        AES128_RAM_CHANNEL, STAT_ERROR_MASK,
    },
    memlog, memlog_flush_and_reset, util,
};

/// AES-128 channel
pub struct Aes128 {
    key: KeySelect,
    // this struct implicitly owns a shared reference to `HW_DCP` ..
    _not_send_or_sync: PhantomData<*mut ()>,
}

// .. however it accesses different registers so instances are OK to `Send`
unsafe impl Send for Aes128 {}

impl Drop for Aes128 {
    fn drop(&mut self) {
        match self.key {
            KeySelect::Key0 => RAM_AES_IN_USE.store(false, Ordering::Release),
            KeySelect::UniqueKey | KeySelect::OtpKey => {
                HW_AES_IN_USE.store(false, Ordering::Release)
            }
        }
    }
}

impl BlockCipher for Aes128 {
    type KeySize = consts::U16;
    type BlockSize = consts::U16;
    type ParBlocks = consts::U1;

    fn new(key: &GenericArray<u8, consts::U16>) -> Self {
        Self::new_ram(key).expect("the `Aes128` is currently in use")
    }

    fn decrypt_block(&self, block: &mut GenericArray<u8, consts::U16>) {
        self.xcrypt(block, false)
    }

    fn encrypt_block(&self, block: &mut GenericArray<u8, consts::U16>) {
        self.xcrypt(block, true)
    }
}

static HW_AES_IN_USE: AtomicBool = AtomicBool::new(false);
static RAM_AES_IN_USE: AtomicBool = AtomicBool::new(false);

enum HardwareKey {
    Unique,
    Otp,
}

impl Into<KeySelect> for HardwareKey {
    fn into(self: HardwareKey) -> KeySelect {
        match self {
            HardwareKey::Unique => KeySelect::UniqueKey,
            HardwareKey::Otp => KeySelect::OtpKey,
        }
    }
}

impl Aes128 {
    /// Gets a handle to the AES-128 channel and configures it to use the unreadable OTP key
    ///
    /// This function returns `None` if the channel is currently in use
    ///
    /// **WARNING!** This routine does NOT check if the OTP was set to an all-zeros value
    pub fn new_otp() -> Option<Self> {
        Self::new_hardware(HardwareKey::Otp)
    }

    /// Gets a handle to the AES-128 channel and configures it to use the unreadable UNIQUE key
    ///
    /// This function returns `None` if the channel is currently in use
    ///
    /// This UNIQUE key is generated from the OTP key and a device-specific 64-bit value
    pub fn new_unique() -> Option<Self> {
        Self::new_hardware(HardwareKey::Unique)
    }

    // Creates a cipher that uses one of the two available hardware keys
    fn new_hardware(key: HardwareKey) -> Option<Self> {
        if HW_AES_IN_USE
            .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            dcp::init();

            HW_DCP::borrow_unchecked(|dcp| {
                const STAT_OTP_KEY_READY: u32 = 1 << 28;

                // check that the OTP is ready to use ("has been released")
                let mut stat = 0;
                let is_ready = || {
                    stat = dcp.STAT.read();
                    stat & STAT_OTP_KEY_READY != 0
                };
                if util::wait_for_or_timeout(is_ready, dcp::default_timeout()).is_err() {
                    memlog!("OTP key not released within timeout (STAT={:#010x})", stat);
                    memlog_flush_and_reset!()
                }

                // enable channel #3
                // NOTE single instruction write to a stateless register
                dcp.CHANNELCTRL_SET.write(1 << AES128_HW_CHANNEL);
            });

            Some(Aes128 {
                key: key.into(),
                _not_send_or_sync: PhantomData,
            })
        } else {
            None
        }
    }

    fn new_ram(key: &GenericArray<u8, consts::U16>) -> Option<Self> {
        if RAM_AES_IN_USE
            .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            dcp::init();

            HW_DCP::borrow_unchecked(|dcp| {
                // install key in the write-only register
                // NOTE to support multiple cipher instances this would need a mutex (e.g. spinlock)
                // to do a exclusive write to these registers
                dcp.KEY.write(u32::from(AES128_RAM_CHANNEL) << 6);
                dcp.KEYDATA
                    .write(u32::from_le_bytes(*array_ref!(key, 0, 4)));
                dcp.KEYDATA
                    .write(u32::from_le_bytes(*array_ref!(key, 4, 4)));
                dcp.KEYDATA
                    .write(u32::from_le_bytes(*array_ref!(key, 8, 4)));
                dcp.KEYDATA
                    .write(u32::from_le_bytes(*array_ref!(key, 12, 4)));

                // enable channel
                // NOTE single instruction write to a stateless register
                // NOTE(| 1 << 3) this is not documented (silicon bug? software restriction?) but
                // appears that a higher numbered channel needs to be enabled for this channel to
                // work (probably the hardware expects the channels to be enabled from higher
                // numbered to lower numbered)
                dcp.CHANNELCTRL_SET
                    .write((1 << AES128_RAM_CHANNEL) | (1 << 3));
            });

            Some(Aes128 {
                key: KeySelect::Key0,
                _not_send_or_sync: PhantomData,
            })
        } else {
            None
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
            if util::wait_for_or_timeout(is_done_or_error, dcp::default_timeout()).is_err() {
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
