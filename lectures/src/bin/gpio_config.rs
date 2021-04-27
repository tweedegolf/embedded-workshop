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
    // Intialize pin p0.11 as pull up input
    let button_1_pin = port0.p0_11.into_pullup_input();

    loop {
        if button_1_pin.is_low().unwrap() {
            // Buttons and LEDs are active low on our board,
            // so LED1 switches on when pressing BTN1
            led_1_pin.set_low();
        } else {
            // LED1 is switched off if BTN1 is not pressed
            led_1_pin.set_high();
        }
    }
    // ANCHOR_END: init
}
