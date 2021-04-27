/* ANCHOR: all */
// ANCHOR: attributes
#![no_std]
#![no_main]
// ANCHOR_END: attributes

// ANCHOR: entry
use cortex_m_rt::entry;

#[entry]
fn start() -> ! {
    loop {}
}
// ANCHOR_END: entry

// ANCHOR: panic_handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_ef: &PanicInfo) -> ! {
    loop {}
}
// ANCHOR_END: panic_handler
