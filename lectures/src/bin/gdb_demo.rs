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


struct Data {
    x: u32,
    y: &'static str,
    z: Option<u8>,
}

#[entry]
fn start() -> ! {
    // Get a handle to te Peripherals

    // Initialize LED pin as output
    // Initialize button pun as input

    let mut x: usize = 0;
    let d = Data {
        x: 123,
        y: "Hello, world!",
        z: Some(123),
    };

    let f = &d;
    
    loop {
        x += 1;
        // TODO
    }
}

#[panic_handler]
fn panic(_ef: &PanicInfo) -> ! {
    loop {}
}
