/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: hal_pac_import
use hal::pac;
use nrf52840_hal as hal;
// ANCHOR_END: hal_pac_import

// ANCHOR: prelude
// Contains all kinds of nice extension traits
use hal::prelude::*;
// ANCHOR_END: prelude

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use examples as _;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M common peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();

    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: init
    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Initialize pin p0.13 as push-pull output, and set it to high state
    let mut led_1_pin = port0.p0_13.into_push_pull_output(hal::gpio::Level::Low);

    // Initialize TIMER0 as a periodic timer
    let mut timer0 = hal::timer::Timer::periodic(peripherals.TIMER0);

    // Set TIMER0 interval to 500ms
    let delay_us = 500_000u32; // 500 milliseconds
    timer0.start(delay_us);

    loop {
        // This is still busy waiting, but at least it's more precise
        while let Err(_) = timer0.wait() {}
        led_1_pin.set_low();

        while let Err(_) = timer0.wait() {}
        led_1_pin.set_high();
    }
    // ANCHOR_END: init
}
