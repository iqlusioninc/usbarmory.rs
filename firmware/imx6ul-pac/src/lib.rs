#![doc = r" Peripheral Access Crate"]
#![doc = r""]
#![doc = r" Automatically generated. Do not directly modify the source code."]
#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
#[doc = r" A peripheral instance"]
pub trait Peripheral {
    #[doc = r" The base address of this peripheral instance"]
    const BASE_ADDRESS: usize;
}
#[allow(non_snake_case)]
#[doc = "GPIO"]
pub mod gpio {
    use crate::Peripheral;
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_sync: PhantomData<*mut ()>,
        #[doc = "GPIO data register"]
        pub DR: DR<P>,
        #[doc = "GPIO direction register"]
        pub GDIR: GDIR<P>,
        #[doc = "GPIO pad status register"]
        pub PSR: PSR<P>,
        #[doc = "GPIO interrupt configuration register1"]
        pub ICR1: ICR1<P>,
        #[doc = "GPIO interrupt configuration register2"]
        pub ICR2: ICR2<P>,
        #[doc = "GPIO interrupt mask register"]
        pub IMR: IMR<P>,
        #[doc = "GPIO interrupt status register"]
        pub ISR: ISR<P>,
        #[doc = "GPIO edge select register"]
        pub EDGE_SEL: EDGE_SEL<P>,
    }
    unsafe impl<P> Send for Registers<P> where P: Peripheral {}
    #[doc = "GPIO data register"]
    #[allow(non_camel_case_types)]
    pub struct DR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> DR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "GPIO direction register"]
    #[allow(non_camel_case_types)]
    pub struct GDIR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> GDIR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "GPIO pad status register"]
    #[allow(non_camel_case_types)]
    pub struct PSR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> PSR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x08;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "GPIO interrupt configuration register1"]
    #[allow(non_camel_case_types)]
    pub struct ICR1<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ICR1<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "GPIO interrupt configuration register2"]
    #[allow(non_camel_case_types)]
    pub struct ICR2<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ICR2<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x10;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "GPIO interrupt mask register"]
    #[allow(non_camel_case_types)]
    pub struct IMR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> IMR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x14;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "GPIO interrupt status register"]
    #[allow(non_camel_case_types)]
    pub struct ISR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ISR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x18;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn clear(&self, mask: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(mask) }
        }
    }
    #[doc = "GPIO edge select register"]
    #[allow(non_camel_case_types)]
    pub struct EDGE_SEL<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> EDGE_SEL<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x1c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl<P> Registers<P>
    where
        P: Peripheral,
    {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _p: PhantomData,
                _not_sync: PhantomData,
                DR: DR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                GDIR: GDIR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                PSR: PSR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ICR1: ICR1 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ICR2: ICR2 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                IMR: IMR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ISR: ISR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                EDGE_SEL: EDGE_SEL {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[doc = "GPIO1"]
    pub struct _1;
    impl Peripheral for _1 {
        const BASE_ADDRESS: usize = 0x0209_c000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "GPIO1"]
    pub type GPIO1 = Registers<_1>;
    impl GPIO1 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "GPIO2"]
    pub struct _2;
    impl Peripheral for _2 {
        const BASE_ADDRESS: usize = 0x020a_0000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "GPIO2"]
    pub type GPIO2 = Registers<_2>;
    impl GPIO2 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "GPIO3"]
    pub struct _3;
    impl Peripheral for _3 {
        const BASE_ADDRESS: usize = 0x020a_4000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "GPIO3"]
    pub type GPIO3 = Registers<_3>;
    impl GPIO3 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "GPIO4"]
    pub struct _4;
    impl Peripheral for _4 {
        const BASE_ADDRESS: usize = 0x020a_8000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "GPIO4"]
    pub type GPIO4 = Registers<_4>;
    impl GPIO4 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "GPIO5"]
    pub struct _5;
    impl Peripheral for _5 {
        const BASE_ADDRESS: usize = 0x020a_c000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "GPIO5"]
    pub type GPIO5 = Registers<_5>;
    impl GPIO5 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "RNG"]
pub mod rng {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x0228_4000;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "RNGB version ID register"]
        pub VER: VER,
        #[doc = "RNGB command register"]
        pub CMD: CMD,
        #[doc = "RNGB control register"]
        pub CR: CR,
        #[doc = "RNGB status register"]
        pub SR: SR,
        #[doc = "RNGB error status register"]
        pub ESR: ESR,
        #[doc = "RNGB Output FIFO"]
        pub OUT: OUT,
    }
    unsafe impl Send for Registers {}
    #[doc = "RNGB version ID register"]
    #[allow(non_camel_case_types)]
    pub struct VER {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl VER {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x1000_0280;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "RNGB command register"]
    #[allow(non_camel_case_types)]
    pub struct CMD {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CMD {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "RNGB control register"]
    #[allow(non_camel_case_types)]
    pub struct CR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CR {
        const OFFSET: usize = 0x08;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "RNGB status register"]
    #[allow(non_camel_case_types)]
    pub struct SR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SR {
        const OFFSET: usize = 0x0c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_500d;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "RNGB error status register"]
    #[allow(non_camel_case_types)]
    pub struct ESR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl ESR {
        const OFFSET: usize = 0x10;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "RNGB Output FIFO"]
    #[allow(non_camel_case_types)]
    pub struct OUT {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl OUT {
        const OFFSET: usize = 0x14;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                VER: VER {
                    _not_send_or_sync: PhantomData,
                },
                CMD: CMD {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                ESR: ESR {
                    _not_send_or_sync: PhantomData,
                },
                OUT: OUT {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "RNG"]
    pub type RNG = Registers;
    impl RNG {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                VER: VER {
                    _not_send_or_sync: PhantomData,
                },
                CMD: CMD {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                ESR: ESR {
                    _not_send_or_sync: PhantomData,
                },
                OUT: OUT {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "SNVS_LP"]
pub mod snvs_lp {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x020c_c034;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "SNVS_LP Lock Register"]
        pub LR: LR,
        #[doc = "SNVS_LP Control Register"]
        pub CR: CR,
        #[doc = "SNVS_LP Status Register"]
        pub SR: SR,
        #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
        pub SMCMR: SMCMR,
        #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
        pub SMCLR: SMCLR,
        #[doc = "SNVS_LP General-Purpose Register"]
        pub GPR: GPR,
    }
    unsafe impl Send for Registers {}
    #[doc = "SNVS_LP Lock Register"]
    #[allow(non_camel_case_types)]
    pub struct LR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LR {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_LP Control Register"]
    #[allow(non_camel_case_types)]
    pub struct CR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CR {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0020;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_LP Status Register"]
    #[allow(non_camel_case_types)]
    pub struct SR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SR {
        const OFFSET: usize = 0x18;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0008;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
    #[allow(non_camel_case_types)]
    pub struct SMCMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SMCMR {
        const OFFSET: usize = 0x28;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
    #[allow(non_camel_case_types)]
    pub struct SMCLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SMCLR {
        const OFFSET: usize = 0x2c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_LP General-Purpose Register"]
    #[allow(non_camel_case_types)]
    pub struct GPR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl GPR {
        const OFFSET: usize = 0x34;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                LR: LR {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                SMCMR: SMCMR {
                    _not_send_or_sync: PhantomData,
                },
                SMCLR: SMCLR {
                    _not_send_or_sync: PhantomData,
                },
                GPR: GPR {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "SNVS_LP"]
    pub type SNVS_LP = Registers;
    impl SNVS_LP {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                LR: LR {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                SMCMR: SMCMR {
                    _not_send_or_sync: PhantomData,
                },
                SMCLR: SMCLR {
                    _not_send_or_sync: PhantomData,
                },
                GPR: GPR {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "SNVS_HP"]
pub mod snvs_hp {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x020c_c000;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "SNVS_HP Lock register"]
        pub LR: LR,
        #[doc = "SNVS_HP Command register"]
        pub COMR: COMR,
        #[doc = "SNVS_HP Control register"]
        pub CR: CR,
        #[doc = "SNVS_HP Status register"]
        pub SR: SR,
        #[doc = "SNVS_HP Real-Time Counter MSB Register"]
        pub RTCMR: RTCMR,
        #[doc = "SNVS_HP Real-Time Counter LSB Register"]
        pub RTCLR: RTCLR,
        #[doc = "SNVS_HP Time Alarm MSB Register"]
        pub TAMR: TAMR,
        #[doc = "SNVS_HP Time Alarm LSB Register"]
        pub TALR: TALR,
        #[doc = "SNVS_HP Version ID Register 1"]
        pub VIDR1: VIDR1,
        #[doc = "SNVS_HP Version ID Register 2"]
        pub VIDR2: VIDR2,
    }
    unsafe impl Send for Registers {}
    #[doc = "SNVS_HP Lock register"]
    #[allow(non_camel_case_types)]
    pub struct LR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LR {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Command register"]
    #[allow(non_camel_case_types)]
    pub struct COMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl COMR {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Control register"]
    #[allow(non_camel_case_types)]
    pub struct CR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CR {
        const OFFSET: usize = 0x08;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Status register"]
    #[allow(non_camel_case_types)]
    pub struct SR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SR {
        const OFFSET: usize = 0x14;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x8000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Real-Time Counter MSB Register"]
    #[allow(non_camel_case_types)]
    pub struct RTCMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl RTCMR {
        const OFFSET: usize = 0x24;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Real-Time Counter LSB Register"]
    #[allow(non_camel_case_types)]
    pub struct RTCLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl RTCLR {
        const OFFSET: usize = 0x28;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Time Alarm MSB Register"]
    #[allow(non_camel_case_types)]
    pub struct TAMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl TAMR {
        const OFFSET: usize = 0x2c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Time Alarm LSB Register"]
    #[allow(non_camel_case_types)]
    pub struct TALR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl TALR {
        const OFFSET: usize = 0x30;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "SNVS_HP Version ID Register 1"]
    #[allow(non_camel_case_types)]
    pub struct VIDR1 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl VIDR1 {
        const OFFSET: usize = 0x0bf8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x003e_0300;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "SNVS_HP Version ID Register 2"]
    #[allow(non_camel_case_types)]
    pub struct VIDR2 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl VIDR2 {
        const OFFSET: usize = 0x0bfc;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0300_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                LR: LR {
                    _not_send_or_sync: PhantomData,
                },
                COMR: COMR {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                RTCMR: RTCMR {
                    _not_send_or_sync: PhantomData,
                },
                RTCLR: RTCLR {
                    _not_send_or_sync: PhantomData,
                },
                TAMR: TAMR {
                    _not_send_or_sync: PhantomData,
                },
                TALR: TALR {
                    _not_send_or_sync: PhantomData,
                },
                VIDR1: VIDR1 {
                    _not_send_or_sync: PhantomData,
                },
                VIDR2: VIDR2 {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "SNVS_HP"]
    pub type SNVS_HP = Registers;
    impl SNVS_HP {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                LR: LR {
                    _not_send_or_sync: PhantomData,
                },
                COMR: COMR {
                    _not_send_or_sync: PhantomData,
                },
                CR: CR {
                    _not_send_or_sync: PhantomData,
                },
                SR: SR {
                    _not_send_or_sync: PhantomData,
                },
                RTCMR: RTCMR {
                    _not_send_or_sync: PhantomData,
                },
                RTCLR: RTCLR {
                    _not_send_or_sync: PhantomData,
                },
                TAMR: TAMR {
                    _not_send_or_sync: PhantomData,
                },
                TALR: TALR {
                    _not_send_or_sync: PhantomData,
                },
                VIDR1: VIDR1 {
                    _not_send_or_sync: PhantomData,
                },
                VIDR2: VIDR2 {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "UART"]
pub mod uart {
    use crate::Peripheral;
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_sync: PhantomData<*mut ()>,
        #[doc = "UART Receiver Register"]
        pub URXD: URXD<P>,
        #[doc = "UART Transmitter Register"]
        pub UTXD: UTXD<P>,
        #[doc = "UART Control Register 1"]
        pub UCR1: UCR1<P>,
        #[doc = "UART Control Register 2"]
        pub UCR2: UCR2<P>,
        #[doc = "UART Control Register 3"]
        pub UCR3: UCR3<P>,
        #[doc = "UART Control Register 4"]
        pub UCR4: UCR4<P>,
        #[doc = "UART FIFO Control Register"]
        pub UFCR: UFCR<P>,
        #[doc = "UART Status Register 1"]
        pub USR1: USR1<P>,
        #[doc = "UART Status Register 2"]
        pub USR2: USR2<P>,
        #[doc = "UART Escape Character Register"]
        pub UESC: UESC<P>,
        #[doc = "UART Escape Timer Register"]
        pub UTIM: UTIM<P>,
        #[doc = "UART BRM Incremental Register"]
        pub UBIR: UBIR<P>,
        #[doc = "UART BRM Modulator Register"]
        pub UBMR: UBMR<P>,
        #[doc = "UART Baud Rate Count Register"]
        pub UBRC: UBRC<P>,
        #[doc = "UART One Millisecond Register"]
        pub ONEMS: ONEMS<P>,
        #[doc = "UART Test Register"]
        pub UTS: UTS<P>,
        #[doc = "UART RS-485 Mode Control Register"]
        pub UMCR: UMCR<P>,
    }
    unsafe impl<P> Send for Registers<P> where P: Peripheral {}
    #[doc = "UART Receiver Register"]
    #[allow(non_camel_case_types)]
    pub struct URXD<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> URXD<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "UART Transmitter Register"]
    #[allow(non_camel_case_types)]
    pub struct UTXD<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UTXD<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x40;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Control Register 1"]
    #[allow(non_camel_case_types)]
    pub struct UCR1<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UCR1<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x80;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Control Register 2"]
    #[allow(non_camel_case_types)]
    pub struct UCR2<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UCR2<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x84;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0001;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Control Register 3"]
    #[allow(non_camel_case_types)]
    pub struct UCR3<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UCR3<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x88;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0700;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Control Register 4"]
    #[allow(non_camel_case_types)]
    pub struct UCR4<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UCR4<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x8c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_8000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART FIFO Control Register"]
    #[allow(non_camel_case_types)]
    pub struct UFCR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UFCR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x90;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0801;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Status Register 1"]
    #[allow(non_camel_case_types)]
    pub struct USR1<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USR1<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x94;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_2040;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Status Register 2"]
    #[allow(non_camel_case_types)]
    pub struct USR2<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USR2<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x98;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_4028;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Escape Character Register"]
    #[allow(non_camel_case_types)]
    pub struct UESC<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UESC<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x9c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_002b;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Escape Timer Register"]
    #[allow(non_camel_case_types)]
    pub struct UTIM<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UTIM<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xa0;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART BRM Incremental Register"]
    #[allow(non_camel_case_types)]
    pub struct UBIR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UBIR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xa4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART BRM Modulator Register"]
    #[allow(non_camel_case_types)]
    pub struct UBMR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UBMR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xa8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Baud Rate Count Register"]
    #[allow(non_camel_case_types)]
    pub struct UBRC<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UBRC<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xac;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0004;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "UART One Millisecond Register"]
    #[allow(non_camel_case_types)]
    pub struct ONEMS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ONEMS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xb0;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART Test Register"]
    #[allow(non_camel_case_types)]
    pub struct UTS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UTS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xb4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0060;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "UART RS-485 Mode Control Register"]
    #[allow(non_camel_case_types)]
    pub struct UMCR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> UMCR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0xb8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl<P> Registers<P>
    where
        P: Peripheral,
    {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _p: PhantomData,
                _not_sync: PhantomData,
                URXD: URXD {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UTXD: UTXD {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UCR1: UCR1 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UCR2: UCR2 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UCR3: UCR3 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UCR4: UCR4 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UFCR: UFCR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USR1: USR1 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USR2: USR2 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UESC: UESC {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UTIM: UTIM {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UBIR: UBIR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UBMR: UBMR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UBRC: UBRC {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ONEMS: ONEMS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UTS: UTS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                UMCR: UMCR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[doc = "UART1"]
    pub struct _1;
    impl Peripheral for _1 {
        const BASE_ADDRESS: usize = 0x0202_0000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART1"]
    pub type UART1 = Registers<_1>;
    impl UART1 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART2"]
    pub struct _2;
    impl Peripheral for _2 {
        const BASE_ADDRESS: usize = 0x021e_8000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART2"]
    pub type UART2 = Registers<_2>;
    impl UART2 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART3"]
    pub struct _3;
    impl Peripheral for _3 {
        const BASE_ADDRESS: usize = 0x021e_c000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART3"]
    pub type UART3 = Registers<_3>;
    impl UART3 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART4"]
    pub struct _4;
    impl Peripheral for _4 {
        const BASE_ADDRESS: usize = 0x021f_0000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART4"]
    pub type UART4 = Registers<_4>;
    impl UART4 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART5"]
    pub struct _5;
    impl Peripheral for _5 {
        const BASE_ADDRESS: usize = 0x021f_4000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART5"]
    pub type UART5 = Registers<_5>;
    impl UART5 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART6"]
    pub struct _6;
    impl Peripheral for _6 {
        const BASE_ADDRESS: usize = 0x021f_c000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART6"]
    pub type UART6 = Registers<_6>;
    impl UART6 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART7"]
    pub struct _7;
    impl Peripheral for _7 {
        const BASE_ADDRESS: usize = 0x0201_8000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART7"]
    pub type UART7 = Registers<_7>;
    impl UART7 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "UART8"]
    pub struct _8;
    impl Peripheral for _8 {
        const BASE_ADDRESS: usize = 0x0202_4000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "UART8"]
    pub type UART8 = Registers<_8>;
    impl UART8 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "USBNC_USB"]
pub mod usbnc_usb {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x0218_4800;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "USB OTG1 Control Register"]
        pub OTG1_CTRL: OTG1_CTRL,
        #[doc = "USB OTG2 Control Register"]
        pub OTG2_CTRL: OTG2_CTRL,
        #[doc = "OTG1 UTMI PHY Control 0 Register"]
        pub OTG1_PHY_CTRL_0: OTG1_PHY_CTRL_0,
        #[doc = "OTG2 UTMI PHY Control 0 Register"]
        pub OTG2_PHY_CTRL_0: OTG2_PHY_CTRL_0,
    }
    unsafe impl Send for Registers {}
    #[doc = "USB OTG1 Control Register"]
    #[allow(non_camel_case_types)]
    pub struct OTG1_CTRL {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl OTG1_CTRL {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x3000_1000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "USB OTG2 Control Register"]
    #[allow(non_camel_case_types)]
    pub struct OTG2_CTRL {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl OTG2_CTRL {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x3000_1000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "OTG1 UTMI PHY Control 0 Register"]
    #[allow(non_camel_case_types)]
    pub struct OTG1_PHY_CTRL_0 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl OTG1_PHY_CTRL_0 {
        const OFFSET: usize = 0x18;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x8000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "OTG2 UTMI PHY Control 0 Register"]
    #[allow(non_camel_case_types)]
    pub struct OTG2_PHY_CTRL_0 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl OTG2_PHY_CTRL_0 {
        const OFFSET: usize = 0x1c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x8000_0098;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                OTG1_CTRL: OTG1_CTRL {
                    _not_send_or_sync: PhantomData,
                },
                OTG2_CTRL: OTG2_CTRL {
                    _not_send_or_sync: PhantomData,
                },
                OTG1_PHY_CTRL_0: OTG1_PHY_CTRL_0 {
                    _not_send_or_sync: PhantomData,
                },
                OTG2_PHY_CTRL_0: OTG2_PHY_CTRL_0 {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "USBNC_USB"]
    pub type USBNC_USB = Registers;
    impl USBNC_USB {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                OTG1_CTRL: OTG1_CTRL {
                    _not_send_or_sync: PhantomData,
                },
                OTG2_CTRL: OTG2_CTRL {
                    _not_send_or_sync: PhantomData,
                },
                OTG1_PHY_CTRL_0: OTG1_PHY_CTRL_0 {
                    _not_send_or_sync: PhantomData,
                },
                OTG2_PHY_CTRL_0: OTG2_PHY_CTRL_0 {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "USB_UOG"]
pub mod usb_uog {
    use crate::Peripheral;
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_sync: PhantomData<*mut ()>,
        #[doc = "Identification register"]
        pub ID: ID<P>,
        #[doc = "Hardware General"]
        pub HWGENERAL: HWGENERAL<P>,
        #[doc = "Host Hardware Parameters"]
        pub HWHOST: HWHOST<P>,
        #[doc = "Device Hardware Parameters"]
        pub HWDEVICE: HWDEVICE<P>,
        #[doc = "TX Buffer Hardware Parameters"]
        pub HWTXBUF: HWTXBUF<P>,
        #[doc = "RX Buffer Hardware Parameters"]
        pub HWRXBUF: HWRXBUF<P>,
        #[doc = "General Purpose Timer #0 Load"]
        pub GPTIMER0LD: GPTIMER0LD<P>,
        #[doc = "General Purpose Timer #0 Controller"]
        pub GPTIMER0CTRL: GPTIMER0CTRL<P>,
        #[doc = "General Purpose Timer #1 Load"]
        pub GPTIMER1LD: GPTIMER1LD<P>,
        #[doc = "General Purpose Timer #1 Controller"]
        pub GPTIMER1CTRL: GPTIMER1CTRL<P>,
        #[doc = "System Bus Config"]
        pub SBUSCFG: SBUSCFG<P>,
        #[doc = "Capability Registers Length"]
        pub CAPLENGTH: CAPLENGTH<P>,
        #[doc = "Host Controller Interface Version"]
        pub HCIVERSION: HCIVERSION<P>,
        #[doc = "Host Controller Structural Parameters"]
        pub HCSPARAMS: HCSPARAMS<P>,
        #[doc = "Host Controller Capability Parameters"]
        pub HCCPARAMS: HCCPARAMS<P>,
        #[doc = "Device Controller Interface Version"]
        pub DCIVERSION: DCIVERSION<P>,
        #[doc = "Device Controller Capability Parameters"]
        pub DCCPARAMS: DCCPARAMS<P>,
        #[doc = "USB Command Register"]
        pub USBCMD: USBCMD<P>,
        #[doc = "USB Status Register"]
        pub USBSTS: USBSTS<P>,
        #[doc = "Interrupt Enable Register"]
        pub USBINTR: USBINTR<P>,
        #[doc = "USB Frame Index"]
        pub FRINDEX: FRINDEX<P>,
        #[doc = "Frame List Base Address"]
        pub PERIODICLISTBASE: PERIODICLISTBASE<P>,
        #[doc = "Device Address"]
        pub DEVICEADDR: DEVICEADDR<P>,
        #[doc = "Next Asynch. Address"]
        pub ASYNCLISTADDR: ASYNCLISTADDR<P>,
        #[doc = "Endpoint List Address"]
        pub ENDPTLISTADDR: ENDPTLISTADDR<P>,
        #[doc = "Programmable Burst Size"]
        pub BURSTSIZE: BURSTSIZE<P>,
        #[doc = "TX FIFO Fill Tuning"]
        pub TXFILLTUNING: TXFILLTUNING<P>,
        #[doc = "Endpoint NAK"]
        pub ENDPTNAK: ENDPTNAK<P>,
        #[doc = "Endpoint NAK Enable"]
        pub ENDPTNAKEN: ENDPTNAKEN<P>,
        #[doc = "Configure Flag Register"]
        pub CONFIGFLAG: CONFIGFLAG<P>,
        #[doc = "Port Status & Control"]
        pub PORTSC1: PORTSC1<P>,
        #[doc = "On-The-Go Status & control"]
        pub OTGSC: OTGSC<P>,
        #[doc = "USB Device Mode"]
        pub USBMODE: USBMODE<P>,
        #[doc = "Endpoint Setup Status"]
        pub ENDPTSETUPSTAT: ENDPTSETUPSTAT<P>,
        #[doc = "Endpoint Prime"]
        pub ENDPTPRIME: ENDPTPRIME<P>,
        #[doc = "Endpoint Flush"]
        pub ENDPTFLUSH: ENDPTFLUSH<P>,
        #[doc = "Endpoint Status"]
        pub ENDPTSTAT: ENDPTSTAT<P>,
        #[doc = "Endpoint Complete"]
        pub ENDPTCOMPLETE: ENDPTCOMPLETE<P>,
        #[doc = "Endpoint Control0"]
        pub ENDPTCTRL0: ENDPTCTRL0<P>,
        #[doc = "Endpoint Control 1"]
        pub ENDPTCTRL1: ENDPTCTRL1<P>,
        #[doc = "Endpoint Control 2"]
        pub ENDPTCTRL2: ENDPTCTRL2<P>,
        #[doc = "Endpoint Control 3"]
        pub ENDPTCTRL3: ENDPTCTRL3<P>,
        #[doc = "Endpoint Control 4"]
        pub ENDPTCTRL4: ENDPTCTRL4<P>,
        #[doc = "Endpoint Control 5"]
        pub ENDPTCTRL5: ENDPTCTRL5<P>,
        #[doc = "Endpoint Control 6"]
        pub ENDPTCTRL6: ENDPTCTRL6<P>,
        #[doc = "Endpoint Control 7"]
        pub ENDPTCTRL7: ENDPTCTRL7<P>,
    }
    unsafe impl<P> Send for Registers<P> where P: Peripheral {}
    #[doc = "Identification register"]
    #[allow(non_camel_case_types)]
    pub struct ID<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ID<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0xe4a1_fa05;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Hardware General"]
    #[allow(non_camel_case_types)]
    pub struct HWGENERAL<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HWGENERAL<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0035;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Host Hardware Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HWHOST<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HWHOST<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x08;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x1002_0001;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Device Hardware Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HWDEVICE<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HWDEVICE<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0011;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "TX Buffer Hardware Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HWTXBUF<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HWTXBUF<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x10;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x8008_0b08;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "RX Buffer Hardware Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HWRXBUF<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HWRXBUF<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x14;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0808;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "General Purpose Timer #0 Load"]
    #[allow(non_camel_case_types)]
    pub struct GPTIMER0LD<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> GPTIMER0LD<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x80;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "General Purpose Timer #0 Controller"]
    #[allow(non_camel_case_types)]
    pub struct GPTIMER0CTRL<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> GPTIMER0CTRL<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x84;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "General Purpose Timer #1 Load"]
    #[allow(non_camel_case_types)]
    pub struct GPTIMER1LD<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> GPTIMER1LD<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x88;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "General Purpose Timer #1 Controller"]
    #[allow(non_camel_case_types)]
    pub struct GPTIMER1CTRL<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> GPTIMER1CTRL<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x8c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "System Bus Config"]
    #[allow(non_camel_case_types)]
    pub struct SBUSCFG<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> SBUSCFG<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x90;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0002;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Capability Registers Length"]
    #[allow(non_camel_case_types)]
    pub struct CAPLENGTH<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> CAPLENGTH<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0100;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u8 = 0x0000_0040;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u8 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u8).read_volatile() }
        }
    }
    #[doc = "Host Controller Interface Version"]
    #[allow(non_camel_case_types)]
    pub struct HCIVERSION<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HCIVERSION<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0102;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0100;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
    }
    #[doc = "Host Controller Structural Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HCSPARAMS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HCSPARAMS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0104;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0001_0011;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Host Controller Capability Parameters"]
    #[allow(non_camel_case_types)]
    pub struct HCCPARAMS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> HCCPARAMS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0108;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0006;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Device Controller Interface Version"]
    #[allow(non_camel_case_types)]
    pub struct DCIVERSION<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> DCIVERSION<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0120;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0001;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
    }
    #[doc = "Device Controller Capability Parameters"]
    #[allow(non_camel_case_types)]
    pub struct DCCPARAMS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> DCCPARAMS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0124;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0188;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "USB Command Register"]
    #[allow(non_camel_case_types)]
    pub struct USBCMD<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USBCMD<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0140;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0008_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "USB Status Register"]
    #[allow(non_camel_case_types)]
    pub struct USBSTS<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USBSTS<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0144;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[allow(non_camel_case_types)]
    pub struct USBINTR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USBINTR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0148;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "USB Frame Index"]
    #[allow(non_camel_case_types)]
    pub struct FRINDEX<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> FRINDEX<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x014c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Frame List Base Address"]
    #[allow(non_camel_case_types)]
    pub struct PERIODICLISTBASE<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> PERIODICLISTBASE<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0154;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Device Address"]
    #[allow(non_camel_case_types)]
    pub struct DEVICEADDR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> DEVICEADDR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0154;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Next Asynch. Address"]
    #[allow(non_camel_case_types)]
    pub struct ASYNCLISTADDR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ASYNCLISTADDR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0158;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint List Address"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTLISTADDR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTLISTADDR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0158;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Programmable Burst Size"]
    #[allow(non_camel_case_types)]
    pub struct BURSTSIZE<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> BURSTSIZE<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0160;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0808;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "TX FIFO Fill Tuning"]
    #[allow(non_camel_case_types)]
    pub struct TXFILLTUNING<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> TXFILLTUNING<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0164;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x000a_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint NAK"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTNAK<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTNAK<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0178;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint NAK Enable"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTNAKEN<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTNAKEN<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x017c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Configure Flag Register"]
    #[allow(non_camel_case_types)]
    pub struct CONFIGFLAG<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> CONFIGFLAG<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0180;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0001;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Port Status & Control"]
    #[allow(non_camel_case_types)]
    pub struct PORTSC1<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> PORTSC1<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x0184;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x1000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "On-The-Go Status & control"]
    #[allow(non_camel_case_types)]
    pub struct OTGSC<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> OTGSC<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01a4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_1120;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "USB Device Mode"]
    #[allow(non_camel_case_types)]
    pub struct USBMODE<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> USBMODE<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01a8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_5000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Setup Status"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTSETUPSTAT<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTSETUPSTAT<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01ac;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Prime"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTPRIME<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTPRIME<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01b0;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Flush"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTFLUSH<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTFLUSH<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01b4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Status"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTSTAT<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTSTAT<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01b8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "Endpoint Complete"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCOMPLETE<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCOMPLETE<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01bc;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control0"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL0<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL0<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01c0;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0080_0080;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 1"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL1<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL1<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01c4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 2"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL2<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL2<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01c8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 3"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL3<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL3<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01cc;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 4"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL4<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL4<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01d0;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 5"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL5<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL5<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01d4;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 6"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL6<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL6<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01d8;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Endpoint Control 7"]
    #[allow(non_camel_case_types)]
    pub struct ENDPTCTRL7<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> ENDPTCTRL7<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x01dc;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl<P> Registers<P>
    where
        P: Peripheral,
    {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _p: PhantomData,
                _not_sync: PhantomData,
                ID: ID {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HWGENERAL: HWGENERAL {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HWHOST: HWHOST {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HWDEVICE: HWDEVICE {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HWTXBUF: HWTXBUF {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HWRXBUF: HWRXBUF {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                GPTIMER0LD: GPTIMER0LD {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                GPTIMER0CTRL: GPTIMER0CTRL {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                GPTIMER1LD: GPTIMER1LD {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                GPTIMER1CTRL: GPTIMER1CTRL {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                SBUSCFG: SBUSCFG {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                CAPLENGTH: CAPLENGTH {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HCIVERSION: HCIVERSION {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HCSPARAMS: HCSPARAMS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                HCCPARAMS: HCCPARAMS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                DCIVERSION: DCIVERSION {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                DCCPARAMS: DCCPARAMS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USBCMD: USBCMD {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USBSTS: USBSTS {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USBINTR: USBINTR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                FRINDEX: FRINDEX {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                PERIODICLISTBASE: PERIODICLISTBASE {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                DEVICEADDR: DEVICEADDR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ASYNCLISTADDR: ASYNCLISTADDR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTLISTADDR: ENDPTLISTADDR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                BURSTSIZE: BURSTSIZE {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                TXFILLTUNING: TXFILLTUNING {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTNAK: ENDPTNAK {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTNAKEN: ENDPTNAKEN {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                CONFIGFLAG: CONFIGFLAG {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                PORTSC1: PORTSC1 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                OTGSC: OTGSC {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                USBMODE: USBMODE {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTSETUPSTAT: ENDPTSETUPSTAT {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTPRIME: ENDPTPRIME {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTFLUSH: ENDPTFLUSH {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTSTAT: ENDPTSTAT {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCOMPLETE: ENDPTCOMPLETE {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL0: ENDPTCTRL0 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL1: ENDPTCTRL1 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL2: ENDPTCTRL2 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL3: ENDPTCTRL3 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL4: ENDPTCTRL4 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL5: ENDPTCTRL5 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL6: ENDPTCTRL6 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                ENDPTCTRL7: ENDPTCTRL7 {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[doc = "USB_UOG1"]
    pub struct _1;
    impl Peripheral for _1 {
        const BASE_ADDRESS: usize = 0x0218_4000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "USB_UOG1"]
    pub type USB_UOG1 = Registers<_1>;
    impl USB_UOG1 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "USB_UOG2"]
    pub struct _2;
    impl Peripheral for _2 {
        const BASE_ADDRESS: usize = 0x0218_4200;
    }
    #[allow(non_camel_case_types)]
    #[doc = "USB_UOG2"]
    pub type USB_UOG2 = Registers<_2>;
    impl USB_UOG2 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "WDOG"]
pub mod wdog {
    use crate::Peripheral;
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_sync: PhantomData<*mut ()>,
        #[doc = "Watchdog Control Register"]
        pub WCR: WCR<P>,
        #[doc = "Watchdog Service Register"]
        pub WSR: WSR<P>,
        #[doc = "Watchdog Reset Status Register"]
        pub WRSR: WRSR<P>,
        #[doc = "Watchdog Interrupt Control Register"]
        pub WICR: WICR<P>,
        #[doc = "Watchdog Miscellaneous Control Register"]
        pub WMCR: WMCR<P>,
    }
    unsafe impl<P> Send for Registers<P> where P: Peripheral {}
    #[doc = "Watchdog Control Register"]
    #[allow(non_camel_case_types)]
    pub struct WCR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> WCR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0030;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u16) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u16).write_volatile(bits) }
        }
    }
    #[doc = "Watchdog Service Register"]
    #[allow(non_camel_case_types)]
    pub struct WSR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> WSR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x02;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u16) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u16).write_volatile(bits) }
        }
    }
    #[doc = "Watchdog Reset Status Register"]
    #[allow(non_camel_case_types)]
    pub struct WRSR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> WRSR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
    }
    #[doc = "Watchdog Interrupt Control Register"]
    #[allow(non_camel_case_types)]
    pub struct WICR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> WICR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x06;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0004;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u16) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u16).write_volatile(bits) }
        }
    }
    #[doc = "Watchdog Miscellaneous Control Register"]
    #[allow(non_camel_case_types)]
    pub struct WMCR<P>
    where
        P: Peripheral,
    {
        _p: PhantomData<P>,
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl<P> WMCR<P>
    where
        P: Peripheral,
    {
        const OFFSET: usize = 0x08;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u16 = 0x0000_0001;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u16 {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *const u16).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u16) {
            unsafe { ((P::BASE_ADDRESS + Self::OFFSET) as *mut u16).write_volatile(bits) }
        }
    }
    impl<P> Registers<P>
    where
        P: Peripheral,
    {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _p: PhantomData,
                _not_sync: PhantomData,
                WCR: WCR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                WSR: WSR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                WRSR: WRSR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                WICR: WICR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
                WMCR: WMCR {
                    _p: PhantomData,
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[doc = "WDOG1"]
    pub struct _1;
    impl Peripheral for _1 {
        const BASE_ADDRESS: usize = 0x020b_c000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "WDOG1"]
    pub type WDOG1 = Registers<_1>;
    impl WDOG1 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "WDOG2"]
    pub struct _2;
    impl Peripheral for _2 {
        const BASE_ADDRESS: usize = 0x020c_0000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "WDOG2"]
    pub type WDOG2 = Registers<_2>;
    impl WDOG2 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
    #[doc = "WDOG3"]
    pub struct _3;
    impl Peripheral for _3 {
        const BASE_ADDRESS: usize = 0x021e_4000;
    }
    #[allow(non_camel_case_types)]
    #[doc = "WDOG3"]
    pub type WDOG3 = Registers<_3>;
    impl WDOG3 {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(unsafe { &Registers::new() })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "GICC"]
pub mod gicc {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x00a0_2000;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "CPU Interface Control Register"]
        pub CTLR: CTLR,
        #[doc = "Interrupt Priority Mask Register"]
        pub PMR: PMR,
        #[doc = "Interrupt Acknowledge Register"]
        pub IAR: IAR,
        #[doc = "End of Interrupt Register"]
        pub EOIR: EOIR,
    }
    unsafe impl Send for Registers {}
    #[doc = "CPU Interface Control Register"]
    #[allow(non_camel_case_types)]
    pub struct CTLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CTLR {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Interrupt Priority Mask Register"]
    #[allow(non_camel_case_types)]
    pub struct PMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl PMR {
        const OFFSET: usize = 0x04;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub unsafe fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Interrupt Acknowledge Register"]
    #[allow(non_camel_case_types)]
    pub struct IAR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl IAR {
        const OFFSET: usize = 0x0c;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_03ff;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "End of Interrupt Register"]
    #[allow(non_camel_case_types)]
    pub struct EOIR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl EOIR {
        const OFFSET: usize = 0x10;
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                CTLR: CTLR {
                    _not_send_or_sync: PhantomData,
                },
                PMR: PMR {
                    _not_send_or_sync: PhantomData,
                },
                IAR: IAR {
                    _not_send_or_sync: PhantomData,
                },
                EOIR: EOIR {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "GICC"]
    pub type GICC = Registers;
    impl GICC {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                CTLR: CTLR {
                    _not_send_or_sync: PhantomData,
                },
                PMR: PMR {
                    _not_send_or_sync: PhantomData,
                },
                IAR: IAR {
                    _not_send_or_sync: PhantomData,
                },
                EOIR: EOIR {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
#[allow(non_snake_case)]
#[doc = "GICD"]
pub mod gicd {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 0x00a0_1000;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "Distributor Control Register"]
        pub CTLR: CTLR,
        #[doc = "Interrupt Set-Enable Registers (4 instances)"]
        pub ISENABLER: ISENABLER,
        #[doc = "Interrupt Priority Registers (128 instances)"]
        pub IPRIORITYR: IPRIORITYR,
        #[doc = "Software Generated Interrupt Register"]
        pub SGIR: SGIR,
    }
    unsafe impl Send for Registers {}
    #[doc = "Distributor Control Register"]
    #[allow(non_camel_case_types)]
    pub struct CTLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl CTLR {
        const OFFSET: usize = 0x00;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0x0000_0000;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    #[doc = "Interrupt Set-Enable Registers (4 instances)"]
    #[allow(non_camel_case_types)]
    pub struct ISENABLER {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl ISENABLER {
        const OFFSET: usize = 0x0100;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self, idx: u8) -> u32 {
            assert!(idx < 4u16 as u8);
            unsafe {
                ((BASE_ADDRESS + Self::OFFSET) as *const u32)
                    .add(usize::from(idx))
                    .read_volatile()
            }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub unsafe fn write(&self, idx: u8, bits: u32) {
            assert!(idx < 4u16 as u8);
            unsafe {
                ((BASE_ADDRESS + Self::OFFSET) as *mut u32)
                    .add(usize::from(idx))
                    .write_volatile(bits)
            }
        }
    }
    #[doc = "Interrupt Priority Registers (128 instances)"]
    #[allow(non_camel_case_types)]
    pub struct IPRIORITYR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl IPRIORITYR {
        const OFFSET: usize = 0x0400;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self, idx: u8) -> u8 {
            assert!(idx < 128u16 as u8);
            unsafe {
                ((BASE_ADDRESS + Self::OFFSET) as *const u8)
                    .add(usize::from(idx))
                    .read_volatile()
            }
        }
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub unsafe fn write(&self, idx: u8, bits: u8) {
            assert!(idx < 128u16 as u8);
            unsafe {
                ((BASE_ADDRESS + Self::OFFSET) as *mut u8)
                    .add(usize::from(idx))
                    .write_volatile(bits)
            }
        }
    }
    #[doc = "Software Generated Interrupt Register"]
    #[allow(non_camel_case_types)]
    pub struct SGIR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl SGIR {
        const OFFSET: usize = 0x0f00;
        #[doc = r" Performs a single store operation on the memory-mapped register"]
        #[allow(unused_unsafe)]
        pub fn write(&self, bits: u32) {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *mut u32).write_volatile(bits) }
        }
    }
    impl Registers {
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Creates a singleton from thin air; make sure we"]
        #[doc = r" never hand out two instances of it"]
        unsafe fn new() -> Self {
            Self {
                _not_sync: PhantomData,
                CTLR: CTLR {
                    _not_send_or_sync: PhantomData,
                },
                ISENABLER: ISENABLER {
                    _not_send_or_sync: PhantomData,
                },
                IPRIORITYR: IPRIORITYR {
                    _not_send_or_sync: PhantomData,
                },
                SGIR: SGIR {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "GICD"]
    pub type GICD = Registers;
    impl GICD {
        #[doc = r" Takes the singleton that represents this peripheral instance"]
        pub fn take() -> Option<Self> {
            static TAKEN: AtomicBool = AtomicBool::new(false);
            if TAKEN
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                Some(unsafe { Registers::new() })
            } else {
                None
            }
        }
        #[doc = r" Borrows the singleton without checking if it's"]
        #[doc = r" currently being held by a context"]
        #[doc = r""]
        #[doc = r" **WARNING** this can break Read-Modify-Write"]
        #[doc = r" operations being performed in other contexts."]
        pub fn borrow_unchecked<T>(f: impl FnOnce(&Self) -> T) -> T {
            f(&Registers {
                _not_sync: PhantomData,
                CTLR: CTLR {
                    _not_send_or_sync: PhantomData,
                },
                ISENABLER: ISENABLER {
                    _not_send_or_sync: PhantomData,
                },
                IPRIORITYR: IPRIORITYR {
                    _not_send_or_sync: PhantomData,
                },
                SGIR: SGIR {
                    _not_send_or_sync: PhantomData,
                },
            })
        }
    }
}
