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
    use hal::uarte::{Baudrate, Parity, Pins};

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Receiving pin, initialize as input
    let rxd = port0.p0_30.into_floating_input().degrade();

    // Transmitting pin, initialize as output
    let txd = port0
        .p0_31
        .into_push_pull_output(hal::gpio::Level::Low)
        .degrade(); // Erase the type, creating a generic pin

    // Create Pins struct to pass to Uarte
    let uart_pins = Pins {
        rxd,
        txd,
        // We don't use this stuff
        cts: None, // Clear to send pin
        rts: None, // Request to send pin
    };

    // Initialize UART peripheral with standard configuration
    // ANCHOR: uarte_init
    let mut uart = hal::Uarte::new(
        peripherals.UARTE0, // Take peripheral handle by value
        uart_pins,          // Take pins by value
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    // ANCHOR_END: uarte_init

    // `Uarte` implements `core::fmt::Write`, which makes writing to it very easy
    use core::fmt::Write;
    write!(uart, "Rust").unwrap();
    // ANCHOR_END: init
    loop {
        // Do nothing
    }
}
