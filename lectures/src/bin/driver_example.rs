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

// ANCHOR: mod_cool_laser_machine
mod cool_laser_machine {
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::blocking::spi::{Write, Transfer};

    use core::marker::PhantomData;

    /// Represents a Cool Laser Machine, that is connected
    /// to some SPI, and to some NCS pin.
    pub struct CoolLaserMachine<SPI, NCS> {
        spi: SPI,
        ncs: NCS,
    }

    /// We only implmenent for `SPI`s that are `Write<u8>` and `Transfer<u8>`,
    /// and for `NCS`s that are `OutputPin`
    impl<SPI, NCS, ESPI, ENCS> CoolLaserMachine<SPI, NCS>
    where
        SPI: Write<u8, Error=ESPI> + Transfer<u8, Error=ESPI>,
        NCS: OutputPin<Error=ENCS>,
        ENCS: core::fmt::Debug, // Allows us to call `unwrap`, which is naughty
    {
        /// Instantiate a Cool Laser Machine, taking ownership
        /// of the SPI peripheral and the NCS pin
        pub fn new(spi: SPI, mut ncs: NCS) -> Self {
            // Ensure the NCS pin is set high
            ncs.set_high();
            Self {
                spi,
                ncs
            }
        }

        /// Puts the LASER on
        pub fn laser_on(&mut self) {
            self.ncs.set_low().unwrap();
            self.spi.write(b"LASER ON!");
            self.ncs.set_high().unwrap();
        }

        /// Switches the LASER off
        pub fn laser_off(&mut self) {
            self.ncs.set_low().unwrap();
            self.spi.write(b"LASER OFF!");
            self.ncs.set_high().unwrap();
        }

        /// Reads the LASER status
        pub fn laser_status(&mut self) -> Result<bool, ESPI> {
            let mut msg = *b"LASER STATUS";
            self.ncs.set_low().unwrap();
            let status = self.spi.transfer(&mut msg)?;
            self.ncs.set_high().unwrap();
            Ok(status[0] != 0)
        }
    }
}
// ANCHOR_END: mod_cool_laser_machine

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

    // SPI mode. We could also use hal::spim::MODE_0 here.
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    // Initialize SPIM peripheral with 500 khz clock frequency
    // ANCHOR: spim_init
    let mut spi = hal::Spim::new(
        peripherals.SPIM0, // Take peripheral handle by value
        spi_pins, // Take pins by value
        Frequency::K500,
        spi_mode,
        0,
    );
    // ANCHOR_END: spim_init
    // ANCHOR: lasers
    use cool_laser_machine::CoolLaserMachine;
    let mut laser_machine = CoolLaserMachine::new(spi, not_chip_sel);
    laser_machine.laser_on();
    let status = laser_machine.laser_status().unwrap();
    laser_machine.laser_off();
    // ANCHOR_END: lasers
    loop {
        // Do nothing
    }
    // ANCHOR_END: init
}
