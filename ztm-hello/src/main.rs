#![no_std]
#![no_main]

// Panic provider crate
use panic_reset as _;

// Provides definitions for our development board
use dwm1001::{
    cortex_m_rt::entry,
    nrf52832_hal::prelude::*,
    DWM1001
};

#[entry]
fn main() -> ! {
    // Set up the board, initializing the LEDs on the board
    let mut board = DWM1001::take().unwrap();

    // Obtain a microsecond precision timer
    let mut timer = board.TIMER0.constrain();

    loop {
        // board.leds.D10 - Bottom LED BLUE
        board.leds.D10.enable();
        timer.delay(2_000_000);

        board.leds.D10.disable();
        timer.delay(6_000_000);
    }
}
