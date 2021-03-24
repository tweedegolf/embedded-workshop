/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: hal_pac_import
use stm32l4xx_hal as hal;
use hal::stm32 as pac;

// Contains all kinds of nice extension traits
use hal::prelude::*;
// ANCHOR_END: hal_pac_import

use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();
    // Get a handle to the STM32L476RG peripherals
    let _peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // TODO Your initialization code here

    loop {
        // TODO Your main loop code here
    }
}

#[panic_handler]
fn panic(_ef: &PanicInfo) -> ! {
    loop {
        // TODO do something with the info
    }
}