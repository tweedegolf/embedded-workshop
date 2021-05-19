/* ANCHOR: all */
#![no_std]
#![no_main]

use cortex_m::interrupt::{free as interrupt_free, Mutex};
// ANCHOR: hal_pac_import
use hal::{
    gpio::{p0::Parts, Level},
    gpiote::Gpiote,
    pac,
    timer::{Periodic, Timer},
};
use nrf52840_hal as hal;
// ANCHOR_END: hal_pac_import

// ANCHOR: prelude
// Contains all kinds of nice extension traits
use hal::prelude::*;
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

static BUTTON_1_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_1_RELEASED: AtomicBool = AtomicBool::new(true);
static TIMER0_FIRED: AtomicBool = AtomicBool::new(false);

static mut GPIOTE_HANDLE: Mutex<RefCell<MaybeUninit<Gpiote>>> =
    Mutex::new(RefCell::new(MaybeUninit::uninit()));

static mut TIMER0_HANDLE: Mutex<RefCell<MaybeUninit<Timer<pac::TIMER0, Periodic>>>> =
    Mutex::new(RefCell::new(MaybeUninit::uninit()));

#[interrupt]
fn GPIOTE() {
    interrupt_free(|cs| {
        let gpiote = unsafe { &mut *(GPIOTE_HANDLE.borrow(cs).borrow_mut().as_mut_ptr()) };
        if gpiote.channel0().is_event_triggered() {
            BUTTON_1_PRESSED.store(true, Relaxed);
            gpiote.channel0().reset_events();
        }
        if gpiote.channel1().is_event_triggered() {
            BUTTON_1_RELEASED.store(true, Relaxed);
            gpiote.channel1().reset_events();
        }
    });
}

#[interrupt]
fn TIMER0() {
    interrupt_free(|cs| {
        let timer0 = unsafe { &mut *(TIMER0_HANDLE.borrow(cs).borrow_mut().as_mut_ptr()) };

        if timer0.event_compare_cc0().read().bits() != 0x00u32 {
            TIMER0_FIRED.store(true, Relaxed);
            timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) })
        }
    });
}

#[entry]
fn start() -> ! {
    // ANCHOR: peripheral_init
    // Get a handle to the nRF52840 device peripherals
    let peripherals = Peripherals::take().unwrap();
    // ANCHOR_END: peripheral_init

    // ANCHOR: init
    // Initialize port0
    let port0 = Parts::new(peripherals.P0);

    // Initialize pin p0.13 as push-pull output, and set it to high state
    let mut led_1_pin = port0.p0_13.into_push_pull_output(Level::Low);
    let mut led_2_pin = port0.p0_14.into_push_pull_output(Level::High);
    // Intialize pin p0.11 as pull up input
    let button_1_pin = port0.p0_11.into_pullup_input().degrade();

    let gpiote = Gpiote::new(peripherals.GPIOTE);
    gpiote
        .channel0()
        .input_pin(&button_1_pin)
        .hi_to_lo()
        .enable_interrupt();
    gpiote
        .channel1()
        .input_pin(&button_1_pin)
        .lo_to_hi()
        .enable_interrupt();

    let mut timer0 = Timer::periodic(peripherals.TIMER0);

    timer0.enable_interrupt();
    timer0.start(500_000u32);
    timer0.task_start();

    interrupt_free(|cs| {
        unsafe {
            *TIMER0_HANDLE.borrow(cs).as_ptr() = MaybeUninit::new(timer0);
            *GPIOTE_HANDLE.borrow(cs).as_ptr() = MaybeUninit::new(gpiote);
        };
    });

    // Unmask interrupt in NVIC
    unsafe {
        NVIC::unmask(Interrupt::GPIOTE);
        NVIC::unmask(Interrupt::TIMER0);
    }

    loop {
        if BUTTON_1_PRESSED.swap(false, Ordering::Release) {
            led_1_pin.set_low().unwrap();
        }
        if BUTTON_1_RELEASED.swap(false, Ordering::Release) {
            led_1_pin.set_high().unwrap();
        }
        if TIMER0_FIRED.swap(false, Ordering::Release) {
            match led_2_pin.is_set_low().unwrap() {
                false => led_2_pin.set_low().unwrap(),
                true => led_2_pin.set_high().unwrap(),
            };
        }
        cortex_m::asm::wfi();
    }
    // ANCHOR_END: init
}
