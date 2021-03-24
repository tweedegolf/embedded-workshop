/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: pac_import
use stm32l4xx_hal::stm32 as pac;
// ANCHOR_END: pac_import

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();
    // Get a handle to the STM32L476RG peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: pin_init_pac
    // Initialize a pin using the PAC
    // Enable the GPIOA peripheral
    peripherals.RCC.ahb2enr.write(|w| w.gpioaen().set_bit());
    peripherals.RCC.ahb2rstr.write(|w| w.gpioarst().set_bit());
    peripherals.RCC.ahb2rstr.write(|w| w.gpioarst().clear_bit());

    // Set GPIO pin PA5 to output mode
    peripherals
        .GPIOA
        .moder
        .write(|w| w.moder5().output());

    // Set GPIO pin PA5 to push-pull-output mode
    peripherals
        .GPIOA
        .otyper
        .write(|w| w.ot5().push_pull());

    // Set GPIO pin PA5 to high state
    peripherals.GPIOA.odr.write(|w| w.odr5().set_bit());
    // ANCHOR_END: pin_init_pac

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
