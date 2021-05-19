/* ANCHOR: all */
#![no_std]
#![no_main]

use cortex_m::interrupt::{free as interrupt_free, CriticalSection, Mutex};
use hal::prelude::*;
use hal::{
    gpio::{p0::Parts, Level},
    gpiote::Gpiote,
    pac,
    timer::{Periodic, Timer},
};
use nrf52840_hal as hal;
use pac::Interrupt;
use pac::{interrupt, Peripherals, NVIC};
// ANCHOR_END: prelude

use core::{
    cell::RefCell,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicBool,
        Ordering::{self, Relaxed},
    },
};
use cortex_m_rt::entry;

use examples as _;

// Flags for events
static BUTTON_1_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_1_RELEASED: AtomicBool = AtomicBool::new(true);
static TIMER0_FIRED: AtomicBool = AtomicBool::new(false);

// Handle to the GPIOTE peripheral. Uninitialized on reset.
// Must be initialized before use
static mut GPIOTE_HANDLE: Mutex<RefCell<MaybeUninit<Gpiote>>> =
    Mutex::new(RefCell::new(MaybeUninit::uninit()));

// Handle to the TIMER0 peripheral. Uninitialized on reset.
// Must be initialized before use
static mut TIMER0_HANDLE: Mutex<RefCell<MaybeUninit<Timer<pac::TIMER0, Periodic>>>> =
    Mutex::new(RefCell::new(MaybeUninit::uninit()));

#[interrupt]
// GPIOTE interrupt service routine
fn GPIOTE() {
    // SAFETY: we're only borrowing GPIOTE_HANDLE, which is never used
    // outside of this interrupt handler, except for initialization which
    // happens before the GPTIOTE interrupt is unmasked.
    let cs = unsafe { CriticalSection::new() };
    // SAFETY: before GPIOTE interrupt is unmasked, GPIOTE_HANDLE is initialized.
    // Therefore it is always initialized before we reach this code.
    let gpiote = unsafe { &mut *(GPIOTE_HANDLE.borrow(&cs).borrow_mut().as_mut_ptr()) };

    // Check if something happened on channel 0
    if gpiote.channel0().is_event_triggered() {
        // Raise flag that button 1 has been pressed
        BUTTON_1_PRESSED.store(true, Relaxed);
        // Reset events, so as to prevent looping forever
        gpiote.channel0().reset_events();
    }
    // Check if something happened on channel 1
    if gpiote.channel1().is_event_triggered() {
        // Raise flag that button 1 has been released
        BUTTON_1_RELEASED.store(true, Relaxed);
        // Reset events, so as to prevent looping forever
        gpiote.channel1().reset_events();
    }
}

#[interrupt]
// TIMER0 interrupt service routine
fn TIMER0() {
    // SAFETY: we're only borrowing TIMER0_HANDLE, which is never used
    // outside of this interrupt handler, except for initialization which
    // happens before the TIMER0 interrupt is unmasked.
    let cs = unsafe { CriticalSection::new() };
    // SAFETY: before TIMER0 interrupt is unmasked, TIMER0_HANDLE is initialized.
    // Therefore it is always initialized before we reach this code.
    let timer0 = unsafe { &mut *(TIMER0_HANDLE.borrow(&cs).borrow_mut().as_mut_ptr()) };

    // Check whether capture/compare register 0 was reached
    if timer0.event_compare_cc0().read().bits() != 0x00u32 {
        // Raise flag that timer has fired
        TIMER0_FIRED.store(true, Relaxed);
        // Reset cc0, so as to prevent looping forever
        timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) })
    }
}

#[entry]
fn start() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let port0 = Parts::new(peripherals.P0);
    let mut led_1_pin = port0.p0_13.into_push_pull_output(Level::High);
    let mut led_2_pin = port0.p0_14.into_push_pull_output(Level::High);
    let button_1_pin = port0.p0_11.into_pullup_input().degrade();

    // Initialize GPIOTE peripheral
    let gpiote = Gpiote::new(peripherals.GPIOTE);
    // Attach button 1 high-to-low transition to channel 0
    // and enable the corresponding interrupt
    gpiote
        .channel0()
        .input_pin(&button_1_pin)
        .hi_to_lo()
        .enable_interrupt();
    // Attach button 1 low-to-high transition to channel 1
    // and enable the corresponding interrupt
    gpiote
        .channel1()
        .input_pin(&button_1_pin)
        .lo_to_hi()
        .enable_interrupt();

    // Initialize TIMER0 peripheral
    let mut timer0 = Timer::periodic(peripherals.TIMER0);
    // Enable timer interrupt
    timer0.enable_interrupt();
    // Set timer frequency to 2 Hz
    timer0.start(500_000u32);

    // Initialize the TIMER0 and GPIOTE handles, passing the initialized
    // peripherals.
    interrupt_free(|cs: &CriticalSection| {
        // SAFETY: these handles are updated within a critical section
        unsafe {
            *TIMER0_HANDLE.borrow(cs).as_ptr() = MaybeUninit::new(timer0);
            *GPIOTE_HANDLE.borrow(cs).as_ptr() = MaybeUninit::new(gpiote);
        };
    });

    // Unmask interrupts in NVIC,
    // enabling them globally.
    // Before unmasking, interrupts are disabled
    unsafe {
        NVIC::unmask(Interrupt::GPIOTE);
        NVIC::unmask(Interrupt::TIMER0);
    }

    loop {
        // Check whether button 1 has been pressed
        if BUTTON_1_PRESSED.swap(false, Ordering::Release) {
            led_1_pin.set_low().unwrap();
        }
        // Check whether button 1 has been released
        if BUTTON_1_RELEASED.swap(false, Ordering::Release) {
            led_1_pin.set_high().unwrap();
        }
        // Check whether timer 0 has fired
        if TIMER0_FIRED.swap(false, Ordering::Release) {
            match led_2_pin.is_set_low().unwrap() {
                false => led_2_pin.set_low().unwrap(),
                true => led_2_pin.set_high().unwrap(),
            };
        }
        // Wait for interrupt
        // This makes the core go to sleep until
        // an interrupt occurs
        cortex_m::asm::wfi();
    }
}
