/* ANCHOR: all */
#![no_std]
#![no_main]

// ANCHOR: pac_import
use nrf52840_hal::pac;
// ANCHOR_END: pac_import

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the Cortex-M common peripherals
    let _core_peripherals = pac::CorePeripherals::take().unwrap();
    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: pin_init_pac
    // Initialize a pin using the PAC

    // Get a handle to GPIO port P0
    let port_p0 = peripherals.P0;

    // Set output level to low
    port_p0.outclr.write(|w| w.pin13().set_bit());

    // Put the pin in push-pull output mode
    port_p0.pin_cnf[13].write(|w| {
        w.dir().output();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

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
