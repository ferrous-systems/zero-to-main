# Zero to Main - Rust

Explorations of different ways to bootstrap a Rust application on a Cortex-M based MCU.

These examples use a Decawave DWM1001-DEV board, based on a Nordic nRF52832.

The following scenarios are currently covered:

* `from-scratch`: Writing a basic Blinkylight hello world with no external libraries
* `ztm-hello`: A basic Blinkylight hello world, using libraries that provide common functionality

# Dependencies

* Windows, Mac, or Linux version supported by Rust
* A version of Segger JLinkGDBServer
* Rust v1.31.0 or abive

Please refer to the [installation instructions] from the Embedded Rust book for configuring your system

[installation instructions]: https://rust-embedded.github.io/book/intro/install.html

# Building

`cargo build` builds all apps

# Flashing and Running

`cargo run` will build, flash, and begin a GDB debugging section for all applications
