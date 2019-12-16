#![no_std]
#![no_main]

use core::{
    mem::zeroed,
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

// ----------------------------------------------------------------
// This section contains the code regularly provided outside of
// a Rust `main.rs` file. These items will usually be provided by
// libraries, or "crates" in Rust.
// ----------------------------------------------------------------

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: fn() -> ! = reset_handler;

pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut __sbss: u32; // Start of .bss section
        static mut __ebss: u32; // End of .bss section
        static mut __sdata: u32; // Start of .data section
        static mut __edata: u32; // End of .data section
        static __sidata: u32; // Start of .rodata section
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut __sdata;
        let edata: *mut u32 = &mut __edata;
        let mut sidata: *const u32 = &__sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Call user's main function
    main()
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On a panic, loop forever
    loop {
        continue;
    }
}

// A short delay function
fn delay(ticks: usize) {
    static mut DUMMY: usize = 0;

    // Reduce the number of iterations when in debug mode
    #[cfg(debug_assertions)]
    let ticks = ticks / 128;

    for t in 0..ticks {
        // Prevent the optimizer from removing this loop
        unsafe {
            write_volatile(&mut DUMMY, t);
        }
    }
}

// ----------------------------------------------------------------
// This section of code is typical for application code in Rust
// ----------------------------------------------------------------

mod nrf52;

use nrf52::gpio::{Level, Pins};

fn main() -> ! {
    let gpios = Pins::take();
    let mut led = gpios.p0_31;

    led.set_push_pull_output(Level::Low);

    loop {
        led.set_high();
        delay(2_000_000);

        led.set_low();
        delay(6_000_000);
    }
}
