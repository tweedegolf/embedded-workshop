#![no_std]
#![no_main]

use examples as _;
use nrf52840_hal as hal;

use hal::{
    gpio::{p0::Parts, Input, Level, Output, Pin, PullUp, PushPull},
    gpiote::Gpiote,
    pac,
    prelude::*,
    timer::Periodic,
    Timer,
};

use pac::TIMER0;

#[rtic::app(
    device=nrf52840_hal::pac,
    peripherals=true,
)]
const APP: () = {
    struct Resources {
        gpiote: Gpiote,
        timer0: Timer<TIMER0, Periodic>,
        button_1_pin: Pin<Input<PullUp>>,
        led_1_pin: Pin<Output<PushPull>>,
        led_2_pin: Pin<Output<PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let port0 = Parts::new(ctx.device.P0);
        let led_1_pin = port0.p0_13.into_push_pull_output(Level::High).degrade();
        let led_2_pin = port0.p0_14.into_push_pull_output(Level::High).degrade();
        let button_1_pin = port0.p0_11.into_pullup_input().degrade();

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

        init::LateResources {
            gpiote,
            timer0,
            button_1_pin,
            led_1_pin,
            led_2_pin,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(resources = [led_2_pin], priority = 1)]
    fn toggle_led_2(ctx: toggle_led_2::Context) {
        let led_2_pin = ctx.resources.led_2_pin;
        match led_2_pin.is_set_low().unwrap() {
            false => led_2_pin.set_low().unwrap(),
            true => led_2_pin.set_high().unwrap(),
        };
    }

    #[task(resources = [led_1_pin], priority = 2)]
    fn set_led_1(ctx: set_led_1::Context, on: bool) {
        match on {
            true => ctx.resources.led_1_pin.set_low().unwrap(),
            false=> ctx.resources.led_1_pin.set_high().unwrap(),
        }
        
    }

    #[task(binds = GPIOTE, resources = [gpiote], spawn = [set_led_1])]
    fn on_gpiote(ctx: on_gpiote::Context) {
        let gpiote = ctx.resources.gpiote;
        
        if gpiote.channel0().is_event_triggered() {
            gpiote.channel0().reset_events();
            ctx.spawn.set_led_1(true).unwrap();
        }

        if gpiote.channel1().is_event_triggered() {
            gpiote.channel1().reset_events();
            ctx.spawn.set_led_1(false).unwrap();
        }
    }

    #[task(binds = TIMER0, resources = [timer0], spawn = [toggle_led_2])]
    fn on_timer0(ctx: on_timer0::Context) {
        let timer0 = ctx.resources.timer0;
        if timer0.event_compare_cc0().read().bits() != 0x00u32 {
            timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) });
            ctx.spawn.toggle_led_2().unwrap();
        }
    }

    extern "C" {
        fn SWI0_EGU0();
        fn SWI1_EGU1();
        fn SWI2_EGU2();
    }
};
