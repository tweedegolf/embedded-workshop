#![no_main]
#![no_std]

use rtic::app;

#[app(device=nrf52840_hal::pac)]
const APP: () = {};

// ANCHOR: panic_handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_ef: &PanicInfo) -> ! {
    loop {}
}
// ANCHOR_END: panic_handler
