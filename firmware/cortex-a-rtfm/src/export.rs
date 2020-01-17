use core::cell::Cell;

pub use cortex_a::{enable_irq, wfi};
pub use heapless::{consts, i::Queue as iQueue, spsc::Queue};
use rac::gic::{gicc, gicd};
pub use usbarmory_rt::Interrupt;

pub type FQ<N> = Queue<u8, N, u8>;
pub type RQ<T, N> = Queue<(T, u8), N, u8>;

/// (hard-coded) priority bits
const PRIO_BITS: u8 = 5;

/// Lowest priority supported by the hardware
const IDLE_PRIORITY: u8 = 0;

/// Highest task priority supported by the hardware
const MAX_PRIORITY: u8 = (1 << PRIO_BITS) - 1;

pub fn assert_send<T>()
where
    T: Send,
{
}

pub fn assert_sync<T>()
where
    T: Sync,
{
}

/// # Safety
///
/// must run only once; must be executed before IRQ interrupts are unmasked
pub unsafe fn enable_gic() {
    // enable the CPU interface
    gicc::GICC_CTLR.write_volatile(1);

    // enable the distributor
    gicd::GICD_CTLR.write_volatile(1);
}

/// # Safety
///
/// - `ptr` must be a valid pointer
/// - `ceiling` must be the largest priority of all the contexts this will be called in
pub unsafe fn lock<T, R>(
    ptr: *mut T,
    priority: &Priority,
    ceiling: u8,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    let current = priority.get();

    if current < ceiling {
        priority.set(ceiling);

        set_priority_mask(ceiling);

        // make priority mask change effective before we access the resource data
        cortex_a::isb();

        let r = f(&mut *ptr);

        // have all memory operations on the resource data complete before the following write
        cortex_a::dsb();

        set_priority_mask(current);

        priority.set(current);

        r
    } else {
        f(&mut *ptr)
    }
}

// NOTE(safety) single-instruction volatile write
pub fn pend_sgi(sgi: u8) {
    unsafe {
        gicd::GICD_SGIR.write_volatile(0b10 << 24 | (u32::from(sgi) & 0b1111));
    }
}

// To avoid breaking the scheduler the value of `GICC_PMR` at the exit of an
// interrupt handler needs to be the same as the value it was at the interrupt
// entry. `lock` does not read `GICC_PMR`; instead it writes back a value based
// on the value of `Priority`. This can result in `GICC_PMR` having a higher
// value at interrupt exit, which would lead to some tasks never being able to
// start.
#[inline(always)]
pub fn run<F>(priority: u8, f: F)
where
    F: FnOnce(),
{
    if priority == 1 {
        // if the logical priority of this interrupt is `1` then `GICC_PMR` can
        // only be `IDLE_PRIORITY`
        f();
        unsafe { gicc::GICC_PMR.write_volatile(u32::from(logical2hw(IDLE_PRIORITY))) }
    } else {
        let initial = unsafe { gicc::GICC_PMR.read_volatile() };
        f();
        unsafe { gicc::GICC_PMR.write_volatile(initial) }
    }
}

/// # Safety
///
/// - Must only be used before IRQ interrupts are unmasked; otherwise it can
/// break priority-based critical sections
pub unsafe fn set_priority(irq: u16, logical: u8) {
    debug_assert!(logical > IDLE_PRIORITY);

    gicd::GICD_IPRIORITYR
        .add(usize::from(irq))
        .write_volatile(logical2hw(logical))
}

/// # Safety
///
/// - Must only be used before IRQ interrupts are unmasked; otherwise it can
/// break mask-unmask-based critical sections
pub unsafe fn enable_spi(spi: u16) {
    debug_assert!(spi >= 32);

    gicd::GICD_ISENABLER
        .add(usize::from(spi) >> 5)
        .write_volatile(1 << (spi % 32))
}

/// # Safety
///
/// - Can break priority based critical sections, like `lock`
pub unsafe fn set_priority_mask(logical: u8) {
    gicc::GICC_PMR.write_volatile(u32::from(logical2hw(logical)));
}

fn logical2hw(logical: u8) -> u8 {
    debug_assert!(logical <= MAX_PRIORITY);

    ((1 << PRIO_BITS) - logical - 1) << (8 - PRIO_BITS)
}

// Newtype over `Cell` that *forbids* mutation through a shared reference
pub struct Priority {
    inner: Cell<u8>,
}

impl Priority {
    /// # Safety
    ///
    /// - Initial value must match the static priority of the context where
    /// it'll be used
    /// - Only a single instance must exist per interrupt handler
    #[inline(always)]
    pub unsafe fn new(value: u8) -> Self {
        Priority {
            inner: Cell::new(value),
        }
    }

    // these two methods are used by `lock` (see below) but can't be used from the RTFM application
    #[inline(always)]
    fn set(&self, value: u8) {
        self.inner.set(value)
    }

    #[inline(always)]
    fn get(&self) -> u8 {
        self.inner.get()
    }
}
