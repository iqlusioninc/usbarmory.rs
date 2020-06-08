use core::{
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{self, AtomicBool, Ordering},
};

use digest::generic_array::{typenum::consts, GenericArray};
use digest::{BlockInput, FixedOutput, Input, Reset};
use heapless::Vec;
use pac::hw_dcp::HW_DCP;
use typenum::marker_traits::Unsigned;

use crate::{
    dcp::{self, Cmd, Control0, Control1, HashSelect, SHA256_CHANNEL, STAT_ERROR_MASK},
    memlog, memlog_flush_and_reset,
    util::{self, Align4},
};

// The DCP has a 32 *bit* counter and won't accept more than 512 MB of input
const MAX_COUNT: usize = 512 * 1024 * 1024;

type BlockSize = consts::U64;

/// SHA-256 channel
pub struct Sha256 {
    _not_send_or_sync: PhantomData<*const ()>,
    next_is_first_block: bool,
    // We collect partial blocks until we have a complete block
    buffer: Vec<u8, BlockSize>,
    /// Input counter, in *bytes*
    count: usize,
}

unsafe impl Send for Sha256 {}

static SHA_IN_USE: AtomicBool = AtomicBool::new(false);

impl Sha256 {
    /// Gets a handle to the SHA-256 channel
    ///
    /// This function returns `None` if the channel is currently in use
    pub fn take() -> Option<Self> {
        if SHA_IN_USE
            .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            dcp::init();

            // NOTE(borrow_unchecked) single-instruction write to a stateless register
            HW_DCP::borrow_unchecked(|dcp| {
                dcp.CHANNELCTRL_SET.write(1 << SHA256_CHANNEL);
            });

            Some(Sha256 {
                _not_send_or_sync: PhantomData,
                buffer: Vec::new(),
                count: 0,
                next_is_first_block: true,
            })
        } else {
            None
        }
    }

    // non-generic version of `Input::input` to reduce .text size (monomorphizations on `Input`
    // become jumps into this common function)
    fn input_slice(&mut self, input: &[u8]) {
        if input.is_empty() {
            // no-op
            return;
        }

        if self
            .count
            .checked_add(input.len())
            .map(|total| total > MAX_COUNT)
            .unwrap_or(true)
        {
            panic!("Sha256: total input exceeds the DCP capacity");
        }

        self.count += input.len();

        if self.buffer.is_empty() {
            // process leading blocks then store the leftover
            process_and_push(&mut self.next_is_first_block, &mut self.buffer, input);
        } else if self.buffer.len() + input.len() < BlockSize::USIZE {
            // not enough to fill a block
            let _ = self.buffer.extend_from_slice(input);
        } else {
            // complete the partial block and then process the rest
            push_and_process(&mut self.next_is_first_block, &mut self.buffer, input)
        }
    }
}

fn process_and_push(next_is_first_block: &mut bool, buffer: &mut Vec<u8, BlockSize>, input: &[u8]) {
    debug_assert!(buffer.is_empty());

    let leftover = if input.len() >= BlockSize::USIZE {
        let (blocks, leftover) = input.split_at(util::round_down(input.len(), BlockSize::USIZE));

        process_blocks(next_is_first_block, blocks);
        leftover
    } else {
        input
    };

    let _ = buffer.extend_from_slice(leftover);
}

fn push_and_process(next_is_first_block: &mut bool, buffer: &mut Vec<u8, BlockSize>, input: &[u8]) {
    debug_assert!(!buffer.is_empty());

    let (head, tail) = input.split_at(BlockSize::USIZE - buffer.len());

    // complete a block
    let _ = buffer.extend_from_slice(head);
    process_blocks(next_is_first_block, buffer);
    buffer.clear();

    process_and_push(next_is_first_block, buffer, tail)
}

// NOTE only the last block (`hash_term = true`) can be smaller than 512 bits
fn process_blocks(next_is_first_block: &mut bool, blocks: &[u8]) {
    debug_assert!(!blocks.is_empty());
    debug_assert_eq!(blocks.len() % BlockSize::USIZE, 0);

    let cmd = Cmd::new();

    cmd.control0.set(
        *Control0::new()
            .decr_semaphore(true)
            .enable_hash(true)
            .hash_output(false) // hash input data
            .hash_init(*next_is_first_block)
            .hash_term(false),
    );
    *next_is_first_block = false;

    cmd.control1
        .set(*Control1::new().hash_select(HashSelect::Sha256));

    cmd.src_buffer_addr.set(Some(NonNull::from(&blocks[0])));
    // redundant: `Cmd::new` sets `dest_buffer_addr` to `None`
    // cmd.dest_buffer_addr.set(None);
    cmd.buffer_size.set(blocks.len());

    HW_DCP::borrow_unchecked(|dcp| {
        // TODO generalize over channel index
        dcp.CH2CMDPTR.write(&cmd as *const Cmd as u32);

        // the write below transfers ownership of `bytes` to the crypto engine; this fence
        // drives all pending writes to `bytes` to completion
        atomic::fence(Ordering::Release);

        // start encryption
        dcp.CH2SEMA.write(1);

        // wait for channel to signal it's done -- the trait interface requires this operation
        // to be blocking
        let mut stat = 0;
        let mut status = 0;
        let is_done_or_error = || {
            // NOTE(read_volatile) we want this statement to always perform a load
            // instruction rather than read memory once and cache the value in a (CPU)
            // register
            status = unsafe { cmd.status.get().read_volatile() };
            stat = dcp.CH2STAT.read();

            // done or error
            status & 1 != 0 || stat & STAT_ERROR_MASK != 0
        };
        if util::wait_for_or_timeout(is_done_or_error, dcp::default_timeout()).is_err() {
            memlog!("digest timeout (STAT={:#010x})", stat);
            memlog_flush_and_reset!()
        }

        // the crypto engine is done with the `bytes` and has transferred ownership back to us
        atomic::fence(Ordering::Acquire);

        // the interface won't let us report the error so just abort the program
        if stat & STAT_ERROR_MASK != 0 {
            memlog!(
                "digest error (STAT={:#010x}, Cmd.status={:#010x})",
                stat,
                status
            );
            memlog_flush_and_reset!()
        }
    })
}

impl Input for Sha256 {
    fn input<B>(&mut self, bytes: B)
    where
        B: AsRef<[u8]>,
    {
        self.input_slice(bytes.as_ref());
    }
}

impl FixedOutput for Sha256 {
    type OutputSize = consts::U32;

    fn fixed_result(self) -> GenericArray<u8, consts::U32> {
        // word aligned for performance
        let mut output = Align4 {
            inner: GenericArray::from([0; 32]),
        };

        let cmd = Cmd::new();

        cmd.control0.set(
            *Control0::new()
                .decr_semaphore(true)
                .enable_hash(true)
                .hash_output(false) // hash input data
                .hash_init(self.next_is_first_block)
                .hash_term(true),
        );

        cmd.control1
            .set(*Control1::new().hash_select(HashSelect::Sha256));

        if !self.buffer.is_empty() {
            cmd.src_buffer_addr
                .set(Some(NonNull::from(&self.buffer[0])));
            cmd.buffer_size.set(self.buffer.len());
        }

        cmd.payload_pointer
            .set(Some(NonNull::from(&mut output.inner[0])));

        HW_DCP::borrow_unchecked(|dcp| {
            // TODO generalize over channel index
            dcp.CH2CMDPTR.write(&cmd as *const Cmd as u32);

            // the write below transfers ownership of `output` to the crypto engine; this fence
            // drives all pending writes to `output` to completion
            atomic::fence(Ordering::Release);

            // start encryption
            dcp.CH2SEMA.write(1);

            // wait for channel to signal it's done -- the trait interface requires this operation
            // to be blocking
            let mut stat = 0;
            let mut status = 0;
            let is_done_or_error = || {
                // NOTE(read_volatile) we want this statement to always perform a load
                // instruction rather than read memory once and cache the value in a (CPU)
                // register
                status = unsafe { cmd.status.get().read_volatile() };
                stat = dcp.CH2STAT.read();

                // done or error
                status & 1 != 0 || stat & STAT_ERROR_MASK != 0
            };
            if util::wait_for_or_timeout(is_done_or_error, dcp::default_timeout()).is_err() {
                memlog!("digest timeout (STAT={:#010x})", stat);
                memlog_flush_and_reset!()
            }

            // the crypto engine is done with `output` and has transferred ownership back to us
            atomic::fence(Ordering::Acquire);

            // the interface won't let us report the error so just abort the program
            if stat & STAT_ERROR_MASK != 0 {
                memlog!(
                    "digest error (STAT={:#010x}, Cmd.status={:#010x})",
                    stat,
                    status
                );
                memlog_flush_and_reset!()
            }
        });

        // the engine returns data in reverse order (big endian?)
        output.inner.reverse();
        output.inner
    }
}

impl Reset for Sha256 {
    fn reset(&mut self) {
        self.buffer.clear();
        self.next_is_first_block = true;
    }
}

impl BlockInput for Sha256 {
    type BlockSize = consts::U64;
}

impl Drop for Sha256 {
    fn drop(&mut self) {
        // NOTE(borrow_unchecked) single-instruction write to a stateless register
        HW_DCP::borrow_unchecked(|dcp| {
            dcp.CHANNELCTRL_CLR.write(1 << SHA256_CHANNEL);
        });

        SHA_IN_USE.store(false, Ordering::Release);
    }
}
