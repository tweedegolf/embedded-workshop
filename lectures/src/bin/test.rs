#![no_std]
#![no_main]

use hal::{gpio::Level, pac};
use nrf52840_hal as hal;

use ssd1306::{prelude::*, Builder};

// Contains all kinds of nice extension traits
use hal::prelude::*;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use examples::{self as _, exit};

#[entry]
fn start() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let port1 = hal::gpio::p1::Parts::new(peripherals.P1);

    let twim0_scl = port0.p0_27.into_floating_input().degrade();
    let twim0_sda = port0.p0_26.into_floating_input().degrade();
    let i2c = hal::twim::Twim::new(
        peripherals.TWIM0,
        hal::twim::Pins {
            scl: twim0_scl,
            sda: twim0_sda,
        },
        hal::twim::Frequency::K400,
    );

    let mut lis3dh = lis3dh::Lis3dh::new(i2c, lis3dh::SlaveAddr::Default).unwrap();

    defmt::info!(
        "Found Lis3dh. Device id: {}",
        lis3dh.get_device_id().unwrap()
    );

    let ssd1306_dc = port1.p1_05.into_push_pull_output(Level::Low).degrade();
    let ssd1306_cs = port1.p1_06.into_push_pull_output(Level::High).degrade();
    let mut ssd1306_res = port1.p1_04.into_push_pull_output(Level::High).degrade();
    let spim1_sck = port1.p1_07.into_push_pull_output(Level::Low).degrade();
    let spim1_mosi = port1.p1_08.into_push_pull_output(Level::Low).degrade();

    let spim1 = hal::spim::Spim::new(
        peripherals.SPIM1,
        hal::spim::Pins {
            sck: spim1_sck,
            mosi: Some(spim1_mosi),
            miso: None,
        },
        hal::spim::Frequency::K250,
        embedded_hal::spi::MODE_0,
        0,
    );

    delay_millis(100);
    ssd1306_res.set_high().unwrap();
    delay_millis(100);
    let interface = SPIInterface::new(spim1, ssd1306_dc, ssd1306_cs);
    let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.clear().unwrap();

    defmt::info!("Showing acc measurements on SSD1306 display. Press CTRL+C to quit");
    loop {
        use core::fmt::Write;
        use lis3dh::accelerometer::Accelerometer;

        let sample = lis3dh.accel_norm().unwrap();
        write!(
            &mut disp,
            "X: {:.3}\nY: {:.3}\nZ: {:.3}",
            sample.x, sample.y, sample.z
        )
        .unwrap();
        delay_millis(50);
        disp.clear().unwrap();
    }
}

/// Blockingly delay computation for a given amount of milliseconds
pub fn delay_millis(ms: u32) {
    // `cortex_m::asm::delay`  takes an of clock cycles
    // for which to wait. As our nRF runst on a 64Mhz clock
    // frequency, we multiply the amount of milliseconds by 64000
    cortex_m::asm::delay(ms * 64000);
}
