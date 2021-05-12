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
    // TODO get handle to peripherals.

    // TODO init port

    // TODO init a NCS pin, in push-pull output mode with initial level `High`

    // TODO init pins (any pin works for this exercise, really)

    let spi_mode = hal::spim::MODE_0;
    // TODO init SPIM1 peripheral using `spi_mode`, with a 500kHz clock frequency.
    // pass 0x55 for orc (over-read character)

    // TODO copy a message from program memory to RAM

    // TODO write the message over the SPI bus

    loop {
        // Do nothing
    }
}
