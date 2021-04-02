#![no_std]
#![no_main]

use hal::stm32 as pac;
use stm32l4xx_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;

use core::panic::PanicInfo;
use cortex_m_rt::{entry};

// Contains definitions needed for defmt,
// so we can print messages.
// Also contains a panic handler,
// so we don't need to add it here anymore
use assignments as _;

#[entry]
fn start() -> ! {
    // Get a handle to te Peripherals

    // Initialize LED pin as output
    // Initialize button pun as input

    defmt::info!("Hello, world");
    loop {
        // TODO Read input pin state, checking if button was pressed
    }
}