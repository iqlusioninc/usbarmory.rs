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
        const OFFSET: usize = 0u32 as usize;
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
        const OFFSET: usize = 4u32 as usize;
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
        const OFFSET: usize = 8u32 as usize;
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
        const OFFSET: usize = 12u32 as usize;
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
        const OFFSET: usize = 16u32 as usize;
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
        const OFFSET: usize = 20u32 as usize;
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
        const OFFSET: usize = 24u32 as usize;
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
        const OFFSET: usize = 28u32 as usize;
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
        const BASE_ADDRESS: usize = 34193408u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 34209792u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 34226176u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 34242560u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 34258944u32 as usize;
    }
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
#[doc = "SNVS"]
pub mod snvs {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };
    const BASE_ADDRESS: usize = 34390016u32 as usize;
    #[doc = r" The registers that make up the peripheral"]
    #[allow(non_snake_case)]
    pub struct Registers {
        _not_sync: PhantomData<*mut ()>,
        #[doc = "SNVS_HP Lock register"]
        pub HPLR: HPLR,
        #[doc = "SNVS_HP Command register"]
        pub HPCOMR: HPCOMR,
        #[doc = "SNVS_HP Control register"]
        pub HPCR: HPCR,
        #[doc = "SNVS_HP Status register"]
        pub HPSR: HPSR,
        #[doc = "SNVS_HP Real-Time Counter MSB Register"]
        pub HPRTCMR: HPRTCMR,
        #[doc = "SNVS_HP Real-Time Counter LSB Register"]
        pub HPRTCLR: HPRTCLR,
        #[doc = "SNVS_HP Time Alarm MSB Register"]
        pub HPTAMR: HPTAMR,
        #[doc = "SNVS_HP Time Alarm LSB Register"]
        pub HPTALR: HPTALR,
        #[doc = "SNVS_LP Lock Register"]
        pub LPLR: LPLR,
        #[doc = "SNVS_LP Control Register"]
        pub LPCR: LPCR,
        #[doc = "SNVS_LP Status Register"]
        pub LPSR: LPSR,
        #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
        pub LPSMCMR: LPSMCMR,
        #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
        pub LPSMCLR: LPSMCLR,
        #[doc = "SNVS_LP General-Purpose Register"]
        pub LPGPR: LPGPR,
        #[doc = "SNVS_HP Version ID Register 1"]
        pub HPVIDR1: HPVIDR1,
        #[doc = "SNVS_HP Version ID Register 2"]
        pub HPVIDR2: HPVIDR2,
    }
    unsafe impl Send for Registers {}
    #[doc = "SNVS_HP Lock register"]
    #[allow(non_camel_case_types)]
    pub struct HPLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPLR {
        const OFFSET: usize = 0u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPCOMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPCOMR {
        const OFFSET: usize = 4u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPCR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPCR {
        const OFFSET: usize = 8u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPSR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPSR {
        const OFFSET: usize = 20u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 2147483648u32 as u32;
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
    pub struct HPRTCMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPRTCMR {
        const OFFSET: usize = 36u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPRTCLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPRTCLR {
        const OFFSET: usize = 40u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPTAMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPTAMR {
        const OFFSET: usize = 44u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPTALR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPTALR {
        const OFFSET: usize = 48u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    #[doc = "SNVS_LP Lock Register"]
    #[allow(non_camel_case_types)]
    pub struct LPLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPLR {
        const OFFSET: usize = 52u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct LPCR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPCR {
        const OFFSET: usize = 56u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 32u32 as u32;
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
    pub struct LPSR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPSR {
        const OFFSET: usize = 76u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 8u32 as u32;
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
    pub struct LPSMCMR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPSMCMR {
        const OFFSET: usize = 92u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct LPSMCLR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPSMCLR {
        const OFFSET: usize = 96u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct LPGPR {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl LPGPR {
        const OFFSET: usize = 104u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
    pub struct HPVIDR1 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPVIDR1 {
        const OFFSET: usize = 3064u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 4064000u32 as u32;
        #[doc = r" Performs a single load operation on the memory-mapped register"]
        pub fn read(&self) -> u32 {
            unsafe { ((BASE_ADDRESS + Self::OFFSET) as *const u32).read_volatile() }
        }
    }
    #[doc = "SNVS_HP Version ID Register 2"]
    #[allow(non_camel_case_types)]
    pub struct HPVIDR2 {
        _not_send_or_sync: PhantomData<*mut ()>,
    }
    impl HPVIDR2 {
        const OFFSET: usize = 3068u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 50331648u32 as u32;
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
                HPLR: HPLR {
                    _not_send_or_sync: PhantomData,
                },
                HPCOMR: HPCOMR {
                    _not_send_or_sync: PhantomData,
                },
                HPCR: HPCR {
                    _not_send_or_sync: PhantomData,
                },
                HPSR: HPSR {
                    _not_send_or_sync: PhantomData,
                },
                HPRTCMR: HPRTCMR {
                    _not_send_or_sync: PhantomData,
                },
                HPRTCLR: HPRTCLR {
                    _not_send_or_sync: PhantomData,
                },
                HPTAMR: HPTAMR {
                    _not_send_or_sync: PhantomData,
                },
                HPTALR: HPTALR {
                    _not_send_or_sync: PhantomData,
                },
                LPLR: LPLR {
                    _not_send_or_sync: PhantomData,
                },
                LPCR: LPCR {
                    _not_send_or_sync: PhantomData,
                },
                LPSR: LPSR {
                    _not_send_or_sync: PhantomData,
                },
                LPSMCMR: LPSMCMR {
                    _not_send_or_sync: PhantomData,
                },
                LPSMCLR: LPSMCLR {
                    _not_send_or_sync: PhantomData,
                },
                LPGPR: LPGPR {
                    _not_send_or_sync: PhantomData,
                },
                HPVIDR1: HPVIDR1 {
                    _not_send_or_sync: PhantomData,
                },
                HPVIDR2: HPVIDR2 {
                    _not_send_or_sync: PhantomData,
                },
            }
        }
    }
    #[doc = "SNVS"]
    pub type SNVS = Registers;
    impl SNVS {
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
                HPLR: HPLR {
                    _not_send_or_sync: PhantomData,
                },
                HPCOMR: HPCOMR {
                    _not_send_or_sync: PhantomData,
                },
                HPCR: HPCR {
                    _not_send_or_sync: PhantomData,
                },
                HPSR: HPSR {
                    _not_send_or_sync: PhantomData,
                },
                HPRTCMR: HPRTCMR {
                    _not_send_or_sync: PhantomData,
                },
                HPRTCLR: HPRTCLR {
                    _not_send_or_sync: PhantomData,
                },
                HPTAMR: HPTAMR {
                    _not_send_or_sync: PhantomData,
                },
                HPTALR: HPTALR {
                    _not_send_or_sync: PhantomData,
                },
                LPLR: LPLR {
                    _not_send_or_sync: PhantomData,
                },
                LPCR: LPCR {
                    _not_send_or_sync: PhantomData,
                },
                LPSR: LPSR {
                    _not_send_or_sync: PhantomData,
                },
                LPSMCMR: LPSMCMR {
                    _not_send_or_sync: PhantomData,
                },
                LPSMCLR: LPSMCLR {
                    _not_send_or_sync: PhantomData,
                },
                LPGPR: LPGPR {
                    _not_send_or_sync: PhantomData,
                },
                HPVIDR1: HPVIDR1 {
                    _not_send_or_sync: PhantomData,
                },
                HPVIDR2: HPVIDR2 {
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
        const OFFSET: usize = 0u32 as usize;
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
        const OFFSET: usize = 64u32 as usize;
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
        const OFFSET: usize = 128u32 as usize;
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
        const OFFSET: usize = 132u32 as usize;
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
        const OFFSET: usize = 136u32 as usize;
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
        const OFFSET: usize = 140u32 as usize;
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
        const OFFSET: usize = 144u32 as usize;
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
        const OFFSET: usize = 148u32 as usize;
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
        const OFFSET: usize = 152u32 as usize;
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
        const OFFSET: usize = 156u32 as usize;
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
        const OFFSET: usize = 160u32 as usize;
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
        const OFFSET: usize = 164u32 as usize;
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
        const OFFSET: usize = 168u32 as usize;
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
        const OFFSET: usize = 172u32 as usize;
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
        const OFFSET: usize = 176u32 as usize;
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
        const OFFSET: usize = 180u32 as usize;
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
        const OFFSET: usize = 184u32 as usize;
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
        const BASE_ADDRESS: usize = 33685504u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35553280u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35569664u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35586048u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35602432u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35635200u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 33652736u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 33701888u32 as usize;
    }
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
        const OFFSET: usize = 0u32 as usize;
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
        const OFFSET: usize = 2u32 as usize;
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
        const OFFSET: usize = 4u32 as usize;
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
        const OFFSET: usize = 6u32 as usize;
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
        const OFFSET: usize = 8u32 as usize;
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
        const BASE_ADDRESS: usize = 34324480u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 34340864u32 as usize;
    }
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
        const BASE_ADDRESS: usize = 35536896u32 as usize;
    }
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
    const BASE_ADDRESS: usize = 10493952u32 as usize;
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
        const OFFSET: usize = 0u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
        const OFFSET: usize = 4u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
        const OFFSET: usize = 12u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 1023u32 as u32;
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
        const OFFSET: usize = 16u32 as usize;
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
    const BASE_ADDRESS: usize = 10489856u32 as usize;
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
        const OFFSET: usize = 0u32 as usize;
        #[doc = r" Reset value"]
        pub const RESET_VALUE: u32 = 0u32 as u32;
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
        const OFFSET: usize = 256u32 as usize;
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
        const OFFSET: usize = 1024u32 as usize;
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
        const OFFSET: usize = 3840u32 as usize;
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
