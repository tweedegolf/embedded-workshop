#![no_std]
#![no_main]

use hal::pac;
use nrf52840_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

// Contains definitions needed for defmt,
// so we can print messages.
// Also contains a panic handler,
// so we don't need to add it here anymore
use assignments::{self as _, exit};

#[entry]
fn start() -> ! {
    // Get a handle to te Peripherals

    // Initialize LED pin as output
    // Initialize button pun as input

    // Be sure to check out `assignments::utils::delay_millis()`,
    // it may come in handy for this assignment
    defmt::info!("Hello, world");
    exit();
}
