/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: hal_pac_import
use hal::stm32 as pac;
use stm32l4xx_hal as hal;
// ANCHOR_END: hal_pac_import

// ANCHOR: prelude
// Contains all kinds of nice extension traits
use hal::prelude::*;
// ANCHOR_END: prelude

use core::{any::Any, panic::PanicInfo};
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();
    // Get a handle to the STM32L476RG peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: pin_init_hal
    // Initialize a pin using the HAL

    // Constrain the Reset and Clock Control peripheral
    let mut rcc = peripherals.RCC.constrain();
    // Split the GPIOA block into separate pins
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb2);
    // Initialize pin PA5 as push-pull output, and set it to high state
    let mut hal_pin_pa5 = gpioa.pa5.into_push_pull_output_with_state(
        &mut gpioa.moder,
        &mut gpioa.otyper,
        hal::gpio::State::High,
    );
    // ANCHOR_END: pin_init_hal

    // TODO Your initialization code here
    loop {
        // TODO Your main loop code here
    }
}

#[panic_handler]
fn panic(_ef: &PanicInfo) -> ! {
    loop {
        // TODO do something with the info
    }
}
