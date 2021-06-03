/* ANCHOR: all */
#![no_std]
#![no_main]

use embedded_hal::timer;
use examples as _;
use nrf52840_hal as hal;

use hal::{
    gpio::{p0::Parts, Input, Level, Output, Pin, PullUp, PushPull},
    gpiote::Gpiote,
    pac::TIMER0,
    pac::DWT,
    prelude::*,
    timer::Periodic,
    Timer,
};

// ANCHOR: app_attr
#[rtic::app(
    device=nrf52840_hal::pac,
    peripherals=true,
    monotonic=rtic::cyccnt::CYCCNT
)]
const APP: () = {
// ANCHOR_END: app_attr
    // ANCHOR: resources
    struct Resources {
        gpiote: Gpiote,
        timer0: Timer<TIMER0, Periodic>,
        button_1_pin: Pin<Input<PullUp>>,
        led_1_pin: Pin<Output<PushPull>>,
        led_2_pin: Pin<Output<PushPull>>,
        led_3_pin: Pin<Output<PushPull>>,
        // Initialize `speed` with `0`
        #[init(0)]
        speed: u32,
    }
    // ANCHOR_END: resources

    // ANCHOR: init_start
    // Initialize peripherals, before interrupts are unmasked
    // Returns all resources that need to be dynamically instantiated
    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        // ANCHOR_END: init_start
        let port0 = Parts::new(ctx.device.P0);
        let led_1_pin = port0.p0_13.into_push_pull_output(Level::High).degrade();
        let led_2_pin = port0.p0_14.into_push_pull_output(Level::High).degrade();
        let led_3_pin = port0.p0_15.into_push_pull_output(Level::High).degrade();
        let button_1_pin = port0.p0_11.into_pullup_input().degrade();
        // ANCHOR: cyccnt_enable
        // Enable cycle counter for task scheduling
        ctx.core.DWT.enable_cycle_counter();
        // ANCHOR_END: cyccnt_enable

        let gpiote = Gpiote::new(ctx.device.GPIOTE);
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

        let mut timer0 = Timer::periodic(ctx.device.TIMER0);
        timer0.enable_interrupt();
        timer0.start(500_000u32);
    // ANCHOR: init_end
        init::LateResources {
            gpiote,
            timer0,
            button_1_pin,
            led_1_pin,
            led_2_pin,
            led_3_pin,
        }
    }
    //ANCHOR_END: init_end

    // ANCHOR: idle_task
    // Defines what happens when there's nothing left to do
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep, waiting for an interrupt
            cortex_m::asm::wfi();
        }
    }
    // ANCHOR_END: idle_task

    // ANCHOR: task_toggle_led_1
    /// Software task for toggling LED 1
    #[task(resources = [led_1_pin], priority = 1)]
    fn toggle_led_1(ctx: toggle_led_1::Context) {
        let led_1_pin = ctx.resources.led_1_pin;
        match led_1_pin.is_set_low().unwrap() {
            false => led_1_pin.set_low().unwrap(),
            true => led_1_pin.set_high().unwrap(),
        };
    }
    // ANCHOR_END: task_toggle_led_1

    // ANCHOR: set_led_2_state
    /// Software task for setting LED 2 state
    #[task(resources = [led_2_pin], priority = 3)]
    fn set_led_2_state(ctx: set_led_2_state::Context, on: bool) {
        match on {
            true => ctx.resources.led_2_pin.set_low().unwrap(),
            false => ctx.resources.led_2_pin.set_high().unwrap(),
        }
    }
    // ANCHOR_END: set_led_2_state

    // ANCHOR: set_led_3_state
    /// Software task for setting LED 3 state
    #[task(resources = [led_3_pin], priority = 2, capacity = 2)]
    fn set_led_3_state(ctx: set_led_3_state::Context, on: bool) {
        match on {
            true => ctx.resources.led_3_pin.set_low().unwrap(),
            false => ctx.resources.led_3_pin.set_high().unwrap(),
        }
    }
    // ANCHOR_END: set_led_3_state

    // ANCHOR: on_gpiote
    /// Hardware task for handling GPIOTE events
    #[task(
        binds = GPIOTE, 
        priority = 5,
        resources = [gpiote, timer0, speed], 
        spawn = [set_led_2_state], 
        schedule = [set_led_3_state]
    )]
    fn on_gpiote(ctx: on_gpiote::Context) {
        use rtic::cyccnt::U32Ext;
        let gpiote = ctx.resources.gpiote;
        let timer0 = ctx.resources.timer0;
        let speed: &mut u32 = ctx.resources.speed;

        if gpiote.channel0().is_event_triggered() {
            gpiote.channel0().reset_events();
            ctx.spawn.set_led_2_state(true).ok();
        }

        if gpiote.channel1().is_event_triggered() {
            *speed = (*speed + 1) % 6;
            timer0.start(500_000u32 >> *speed);
            gpiote.channel1().reset_events();

            ctx.spawn.set_led_2_state(false).ok();
            // ANCHOR: schedule
            ctx.schedule.set_led_3_state(ctx.start + 8_000_000.cycles(), true).ok();
            ctx.schedule.set_led_3_state(ctx.start + 32_000_000.cycles(), false).ok();
            // ANCHOR_END: schedule
        }
    }
    // ANCHOR_END: on_gpiote

    // ANCHOR: on_timer0
    /// Hardware task for handling TIMER0 events
    #[task(
        binds = TIMER0,
        priority = 4,
        resources = [timer0, speed], 
        spawn = [toggle_led_1]
    )]
    fn on_timer0(ctx: on_timer0::Context) {
        let mut timer0 = ctx.resources.timer0;
        let fired = timer0.lock(|timer0: &mut Timer<_, _>|{
            if timer0.event_compare_cc0().read().bits() != 0x00u32 {
                timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) });
                return true;
            }
            false
        });
        if fired{     
            ctx.spawn.toggle_led_1().unwrap();
        }
    }
    // ANCHOR_END: on_timer0

    // ANCHOR: extern_interrupts
    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these interrupts will be used to dispatch the
    // software tasks.
    // See https://rtic.rs/0.5/book/en/by-example/tasks.html;
    extern "C" {
        // Software interrupt 0 / Event generator unit 0
        fn SWI0_EGU0();
        // Software interrupt 1 / Event generator unit 1
        fn SWI1_EGU1();
        // Software interrupt 2 / Event generator unit 2
        fn SWI2_EGU2();
    }
    // ANCHOR_END: extern_interrupts
};
