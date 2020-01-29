//! Random Number Generator

// See chapter 44 of the 6ULLRM

use core::cmp;

use pac::rng::RNG;
use rand_core::{block::BlockRngCore, CryptoRng};

/// A Random Number Generator backed by the hardware
///
/// RNG works in two stages: it generates a seed from a TRNG and then feeds that
/// seed to a PRNG to generate random numbers at a rate of 5 32-bit words every
/// 112 clock cycles. After producing `1 << 20` words of random data the PRNG
/// requests a new seed from the TRNG. The initial seed takes approximately 2
/// million clock cycles to produce. Only the output of the PRNG is exposed by
/// the hardware
pub struct Rng {
    inner: RNG,
}

/* Command register */
/// Software reset
const CMD_SR: u32 = 1 << 6;

/* Control Register */
/// Auto-reseed
const CR_AR: u32 = 1 << 4;

/* Status Register */
// Statistics test pass failed.
const SR_STATPF_OFFSET: u8 = 24;
const SR_STATPF_MASK: u32 = 0xff;

/// Seed done
const SR_SDN: u32 = 1 << 5;

// FIFO level
const SR_FIFO_LVL_MASK: u32 = 0b1111;
const SR_FIFO_LVL_OFFSET: u8 = 8;

/// The maximum number of
pub const FIFO_SIZE: usize = 5;

impl Rng {
    /// Gets an exclusive handle to the `Rng` singleton
    ///
    /// This function returns the `Some` variant at most once. If the `pac::RNG`
    /// is taken before calling this function this will return `None`.
    ///
    /// This function will initialize the generation of the first PRNG seed but
    /// it is non-blocking, i.e. it does not wait for the seed generation to
    /// complete.
    pub fn initialize() -> Option<Self> {
        RNG::take().map(|rng| {
            let rng = Self { inner: rng };
            rng.auto_seed();
            rng
        })
    }

    /// [blocking] Returns statistics about the seed being currently used by the
    /// PRNG
    ///
    /// If the first seed has not yet been generated this method will block
    /// until it is generated
    pub fn stats(&self) -> Stats {
        self.wait_for_initial_seed();

        let statpf = (self.inner.SR.read() >> SR_STATPF_OFFSET) & SR_STATPF_MASK;

        Stats {
            monobit_test_failed: statpf & 1 != 0,
            length_1_run_test_failed: statpf & (1 << 1) != 0,
            length_2_run_test_failed: statpf & (1 << 2) != 0,
            length_3_run_test_failed: statpf & (1 << 3) != 0,
            length_4_run_test_failed: statpf & (1 << 4) != 0,
            length_5_run_test_failed: statpf & (1 << 5) != 0,
            length_6_plus_run_test_failed: statpf & (1 << 6) != 0,
            long_run_test: statpf & (1 << 7) != 0,
        }
    }

    /// [blocking] Waits until the first PRNG seed is generated
    pub fn wait_for_initial_seed(&self) {
        while self.inner.SR.read() & SR_SDN == 0 {
            // busy wait
            continue;
        }
    }

    /// [blocking] Extracts one 32-bit word of random data from the PRNG
    ///
    /// This method may block if the internal FIFO buffer of random data is
    /// empty or if the first seed has not yet been generated
    pub fn next_u32(&self) -> u32 {
        self.wait_for_fifo_level(1);
        self.inner.OUT.read()
    }

    /// [blocking] Writes at least one 32-bit word of random data into the
    /// specified `buffer`
    ///
    /// Returns a slice into the written part of the buffer. The length of the
    /// returned slice will never exceed the `FIFO_SIZE` constant
    ///
    /// This method may block if the internal FIFO buffer of random data is
    /// empty or if the first seed has not yet been generated
    pub fn write<'a>(&self, buf: &'a mut [u32]) -> &'a [u32] {
        let n = cmp::min(usize::from(self.wait_for_fifo_level(1)), buf.len());

        for slot in &mut buf[..n] {
            *slot = self.inner.OUT.read();
        }
        &buf[..n]
    }

    /// Enables auto-seeding the PRNG
    fn auto_seed(&self) {
        self.inner.CR.write(CR_AR);
    }

    /// Returns the current level of the FIFO buffer
    fn fifo_level(&self) -> u8 {
        let sr = self.inner.SR.read();
        ((sr >> SR_FIFO_LVL_OFFSET) & SR_FIFO_LVL_MASK) as u8
    }

    /// Software resets the hardware RNG
    fn software_reset(&self) {
        self.inner.CMD.write(CMD_SR);
    }

    /// Waits until the FIFO buffer has at least this many `words`
    ///
    /// Returns the number of 32-bit words that the FIFO buffer currently holds
    ///
    /// This method panics if `words` is greater than `FIFO_SIZE`
    fn wait_for_fifo_level(&self, words: u8) -> u8 {
        assert!(words <= FIFO_SIZE as u8);

        let esr = self.inner.ESR.read();

        if esr != 0 {
            // error detected: software reset the RNG peripheral
            self.software_reset();
            self.auto_seed();
            self.wait_for_initial_seed();
        }

        // busy wait until there's at least `words` of random data in the FIFO
        // buffer
        loop {
            let fifo_level = self.fifo_level();

            if fifo_level >= words {
                break fifo_level;
            }
        }
    }
}

impl CryptoRng for Rng {}

impl BlockRngCore for Rng {
    type Item = u32;
    type Results = [u32; FIFO_SIZE];

    fn generate(&mut self, results: &mut [u32; FIFO_SIZE]) {
        self.wait_for_fifo_level(FIFO_SIZE as u8);

        for slot in results {
            *slot = self.inner.OUT.read();
        }
    }
}

/// The results of running statistics tests
#[derive(Clone, Copy, Debug)]
pub struct Stats {
    /// Monobit test failed
    pub monobit_test_failed: bool,

    /// Length 1 run test failed
    pub length_1_run_test_failed: bool,

    /// Length 2 run test failed
    pub length_2_run_test_failed: bool,

    /// Length 3 run test failed
    pub length_3_run_test_failed: bool,

    /// Length 4 run test failed
    pub length_4_run_test_failed: bool,

    /// Length 5 run test failed
    pub length_5_run_test_failed: bool,

    /// Length 6+ run test failed
    pub length_6_plus_run_test_failed: bool,

    /// Long run test (>34) failed
    pub long_run_test: bool,
}
