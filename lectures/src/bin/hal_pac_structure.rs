/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: hal_pac_import
use nrf52840_hal as hal;
use hal::pac;
// ANCHOR_END: hal_pac_import

// ANCHOR: prelude
// Contains all kinds of nice extension traits
use hal::prelude::*;
// ANCHOR_END: prelude

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M common peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();

    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: pin_init_hal
    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Initialize pin p0.13 as push-pull output, and set it to high state
    let _led_1_pin = port0.p0_13.into_push_pull_output(hal::gpio::Level::Low);
    // ANCHOR_END: pin_init_hal

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
