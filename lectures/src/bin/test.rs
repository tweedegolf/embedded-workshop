#![no_std]
#![no_main]

use hal::pac;
use nrf52840_hal as hal;

// Contains all kinds of nice extension traits
use hal::prelude::*;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use examples::{self as _, exit};

#[entry]
fn start() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    let led_1_pin = port0
        .p0_13
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let led_2_pin = port0
        .p0_14
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let led_3_pin = port0
        .p0_15
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let led_4_pin = port0
        .p0_16
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let button_1_pin = port0.p0_11.into_pullup_input();
    let mut leds = [led_1_pin, led_2_pin, led_4_pin, led_3_pin];

    let mut timer0 = hal::timer::Timer::periodic(peripherals.TIMER0);

    let delay_us = 100_000u32; // 50 milliseconds
    timer0.start(delay_us);

    defmt::info!("Please press Button 1");

    'main: loop {
        for led in leds.iter_mut() {
            led.set_low().unwrap();
            while let Err(_) = timer0.wait() {
                if button_1_pin.is_low().unwrap() {
                    break 'main;
                }
            }
            led.set_high().unwrap();
        }
    }

    defmt::info!("Thanks, goodbye!");

    exit();
}
