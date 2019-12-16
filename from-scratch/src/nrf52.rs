//! A basic simulation of a Rust driver for the nRF52.
//!
//! This is *not* meant to be a good example of how to write
//! low level Rust drivers. Please refer to the `nrf52832-pac`
//! and `nrf52832-hal` crates for idiomatic sources
#![allow(dead_code)]

// A not-very-useful abstraction of GPIOs in Rust
pub mod gpio {
    use core::sync::atomic::{AtomicBool, Ordering::SeqCst};

    /// A struct that represents an nRF52 Pin
    pub struct Pin(u8);

    /// A struct that represents P0 of the nRF52
    pub struct Pins {
        pub p0_31: Pin,
    }

    impl Pins {
        /// A function to obtain a Port 0 singleton structure
        pub fn take() -> Self {
            static TAKEN: AtomicBool = AtomicBool::new(false);

            // Enforce this as a singleton
            assert!(!TAKEN.swap(true, SeqCst));

            Self { p0_31: Pin(31) }
        }
    }

    /// The level of a GPIO
    #[derive(Copy, Clone)]
    pub enum Level {
        Low,
        High,
    }

    const REG_P0_PIN_CNF_BASE: *mut u32 = 0x5000_0700 as *mut u32;
    const REG_P0_OUT_SET: *mut u32 = 0x5000_0508 as *mut u32;
    const REG_P0_OUT_CLR: *mut u32 = 0x5000_050C as *mut u32;

    const PIN_CNF_DIR_OUTPUT: u32 = 0x0000_0001;
    const PIN_CNF_INPUT_CONNECT: u32 = 0x0000_0000;
    const PIN_CNF_PULL_DISABLED: u32 = 0x0000_0000;
    const PIN_CNF_DRIVE_S0S1: u32 = 0x0000_0000;
    const PIN_CNF_SENSE_DISABLED: u32 = 0x0000_0000;

    impl Pin {
        /// Set a pin to be a push pull output
        pub fn set_push_pull_output(&mut self, level: Level) {
            // set level
            match level {
                Level::High => self.set_high(),
                Level::Low => self.set_low(),
            }

            let set_val = PIN_CNF_DIR_OUTPUT
                | PIN_CNF_INPUT_CONNECT
                | PIN_CNF_PULL_DISABLED
                | PIN_CNF_DRIVE_S0S1
                | PIN_CNF_SENSE_DISABLED;

            unsafe {
                core::ptr::write_volatile(REG_P0_PIN_CNF_BASE.offset(self.0 as isize), set_val);
            }
        }

        /// Set a pin to output level low
        pub fn set_low(&mut self) {
            unsafe { core::ptr::write_volatile(REG_P0_OUT_SET, 1 << (self.0 as u32)) }
        }

        /// Set a pin to output level high
        pub fn set_high(&mut self) {
            unsafe { core::ptr::write_volatile(REG_P0_OUT_CLR, 1 << (self.0 as u32)) }
        }
    }
}
