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
    use hal::gpio::Level;
    use hal::twim::{Frequency, Pins};

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Serial data line. Initialize as input.
    let sda = port0.p0_31.into_floating_input().degrade();
    // Serial clock. Initialize as output
    let scl = port0.p0_30.into_floating_input().degrade();

    // Create Pins struct to pass to Spim
    let i2c_pins = Pins { scl, sda };

    // TWIM stands for Two-Wire Interface Master, and it is compatible with I2C
    // ANCHOR: twim_init
    let mut i2c = hal::Twim::new(
        peripherals.TWIM0, // Take peripheral handle by value
        i2c_pins, // Take pins by value
        Frequency::K400,
    );
    // ANCHOR_END: twim_init

    // On our nRF52840, I2C uses DMA, which is why we have to copy
    // the message from flash to RAM.
    let message_flash: &'static [u8; 4] = b"Rust";
    let message_ram = *message_flash; // Copy to RAM

    // Write some data to a slave with address 0x18
    i2c.write(0x18, &message_ram).unwrap();

    loop {
        // Do nothing
    }
    // ANCHOR_END: init
}
