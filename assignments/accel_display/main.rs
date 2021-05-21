#![no_std]
#![no_main]

use core::fmt::write;

use hal::pac;
use nrf52840_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;

// I went ahead and imported a bunch of usefull stuff.
use hal::gpio::Level;
use hal::spim::{Frequency as SpimFrequency, Mode, Phase, Pins as SpimPins, Polarity};
use hal::twim::{Frequency as TwimFrequency, Pins as TwimPins};

use accelerometer::Accelerometer;
use lis3dh::{Lis3dh, SlaveAddr};
use ssd1306::{prelude::*, Builder};

use cortex_m_rt::entry;

use assignments as _;

#[entry]
fn start() -> ! {
    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();

    // Initialize port0 and port1

    // Initialize pins

    // Initialize peripherals

    // Initialize drivers

    loop {
        // Cool stuff
    }
}
