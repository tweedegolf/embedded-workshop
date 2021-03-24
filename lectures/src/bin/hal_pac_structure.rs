/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: hal_pac_import
use hal::stm32 as pac;
use stm32l4xx_hal as hal;
// ANCHOR_END: hal_pac_import

// Contains all kinds of nice extension traits
use hal::prelude::*;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();
    // Get a handle to the STM32L476RG peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: pac_example
    // Set GPIO pin PA0 to high state
    peripherals.GPIOA.odr.write(|w|  w.odr0().set_bit());
    // ANCHOR_END: pac_example

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
