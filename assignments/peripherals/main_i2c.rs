#![no_std]
#![no_main]

use hal::pac;
use nrf52840_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;

use cortex_m_rt::entry;

use assignments as _;

#[entry]
fn start() -> ! {
    // Get a handle to the Cortex-M common peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();

    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();

    use hal::gpio::Level;
    use hal::twim::{Frequency, Pins};

    // Initialize port0
    let port0 = todo!("Initialize port 0");

    // Serial data line. Initialize as input. Any pin is fine
    let sda = todo!("Initialize SDA pin");
    // Serial clock. Initialize as output. Any pin works
    let scl = todo!("Initialize SCL pin");

    // Create Pins struct to pass to Spim
    let i2c_pins = Pins { scl, sda };

    // TWIM stands for Two-Wire Interface Master, and it is compatible with I2C
    let mut i2c: hal::twim::Twim<pac::TWIM0> =
        todo!("Initialize TWIM0 with the SCL and SDA pins, and 500kHz clock frequency");

    // On our nRF52840, I2C uses DMA, which is why we have to copy
    // the message from flash to RAM.
    let message_flash: &'static [u8; 4] = b"Rust";
    let message_ram = *message_flash; // Copy to RAM

    // Write some data to a slave with address 0x18
    todo!("Write data");

    loop {
        // Do nothing
    }
}
