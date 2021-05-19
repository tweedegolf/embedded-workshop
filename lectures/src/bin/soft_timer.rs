/* ANCHOR: all */
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use examples as _;
use hal::pac;
use hal::prelude::*;
use nrf52840_hal as hal;
use void::Void;

fn toggle_pin(pin: &mut impl StatefulOutputPin<Error = Void>) {
    match pin.is_set_low().unwrap() {
        true => pin.set_high().unwrap(),
        false => pin.set_low().unwrap(),
    }
}

#[entry]
fn start() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let mut led_1_pin = port0
        .p0_13
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let mut led_2_pin = port0
        .p0_14
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let mut led_3_pin = port0
        .p0_15
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();
    let mut led_4_pin = port0
        .p0_16
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();

    // ANCHOR: app
    let mut timer = SoftTimer {
        bit_mode: Bitmode::BitWidth16,
        prescaler: 100,
        cc_0_event: || toggle_pin(&mut led_1_pin),
        cc_1_event: || toggle_pin(&mut led_2_pin),
        cc_2_event: || toggle_pin(&mut led_3_pin),
        cc_3_event: || toggle_pin(&mut led_4_pin),
        cc0_compare_value: 0,
        cc1_compare_value: 4000,
        cc2_compare_value: 16000,
        cc3_compare_value: 64000,
        counter: 0,
        ticks: 0,
    };
    loop {
        timer.tick();
    }
    // ANCHOR_END: app
}

#[allow(dead_code)]
// ANCHOR: struct
#[repr(u8)]
#[derive(Clone, Copy)]
enum Bitmode {
    BitWidth16 = 16,
    BitWidth8 = 8,
    BitWidth24 = 24,
    BitWidth32 = 32,
}

struct SoftTimer<F0, F1, F2, F3>
where
    F0: FnMut(),
    F1: FnMut(),
    F2: FnMut(),
    F3: FnMut(),
{
    bit_mode: Bitmode,      // Counter bit width
    prescaler: u32,         // Divides the clock frequency
    cc0_compare_value: u32, // Capture/compare value 0
    cc1_compare_value: u32, // Capture/compare value 1
    cc2_compare_value: u32, // Capture/compare value 2
    cc3_compare_value: u32, // Capture/compare value 3
    cc_0_event: F0,         // Event if cc0 is reached
    cc_1_event: F1,         // Event if cc1 is reached
    cc_2_event: F2,         // Event if cc2 is reached
    cc_3_event: F3,         // Event if cc3 is reached
    counter: u32,           // Current counter value (initially 0)
    ticks: u32,             // Number of ticks since latest counter increment (initially 0)
}
// ANCHOR_END: struct

// ANCHOR: impl
impl<F0, F1, F2, F3> SoftTimer<F0, F1, F2, F3>
where
    F0: FnMut(),
    F1: FnMut(),
    F2: FnMut(),
    F3: FnMut(),
{
    pub fn tick(&mut self) {
        // Divide clock frequency using prescaler
        self.ticks += 1;
        if self.ticks < self.prescaler {
            // Disable optimizing this away
            cortex_m::asm::nop();
            return;
        }
        self.ticks = 0;

        // Compare current counter value
        // and call events accordingly
        match self.counter {
            c if c == self.cc0_compare_value => (self.cc_0_event)(),
            c if c == self.cc1_compare_value => (self.cc_1_event)(),
            c if c == self.cc2_compare_value => (self.cc_2_event)(),
            c if c == self.cc3_compare_value => (self.cc_3_event)(),
            _ => (),
        }

        // Increment counter, wrap on overflow.
        // The bit mode controls when overflows happen
        self.counter = self.counter.wrapping_add(1) & !(2 << self.bit_mode as u8);
    }
}
// ANCHOR_END: impl
