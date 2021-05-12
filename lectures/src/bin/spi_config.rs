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
    use hal::spim::{Frequency, Mode, Phase, Pins, Polarity};

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Chip-select is active low, so we set the initial level to High.
    let mut not_chip_sel = port0.p0_28.into_push_pull_output(Level::High).degrade();

    // Serial clock, intialize as output
    let sck = port0.p0_29.into_push_pull_output(Level::Low).degrade();
    // MOSI, intialize as output
    let mosi = port0.p0_30.into_push_pull_output(Level::Low).degrade();
    // MISO, intialize as input
    let miso = port0.p0_31.into_floating_input().degrade();

    // Create Pins struct to pass to Spim
    let spi_pins = Pins {
        sck,
        mosi: Some(mosi),
        miso: Some(miso),
    };

    // SPI mode.
    let spi_mode = hal::spim::MODE_0;

    // Initialize SPIM peripheral with 500 khz clock frequency
    // ANCHOR: spim_init
    let mut spi = hal::Spim::new(
        peripherals.SPIM0, // Take peripheral handle by value
        spi_pins,          // Take pins by value
        Frequency::K500,
        spi_mode,
        0x55, // Over-read-character
    );
    // ANCHOR_END: spim_init

    // On our nRF52840, SPI uses DMA, which is why we have to copy
    // the message from flash to RAM.
    let message_flash: &'static [u8; 4] = b"Rust";
    let message_ram = *message_flash; // Copy to RAM

    // Write some data, automatically toggling the CS pin.
    spi.write(&mut not_chip_sel, &message_ram).unwrap();
    // ANCHOR_END: init
    loop {
        // Do nothing
    }
}
