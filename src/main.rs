#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32l4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use rtic::time::duration::Seconds;
    use rtic::Monotonic;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32l4xx_hal::lptimer::{
        ClockSource, LowPowerTimer, LowPowerTimerConfig, LowPowerTimerMonotonic,
    };
    use stm32l4xx_hal::{
        prelude::*,
        rcc::{ClockSecuritySystem, CrystalBypass},
    };

    #[monotonic(binds = TIM2, default = true)]
    type MyMono2 = LowPowerTimerMonotonic<32_768>;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut pwr = cx.device.PWR.constrain(&mut rcc.apb1r1);
        let lptim = cx.device.LPTIM1;

        rtt_init_print!(NoBlockSkip, 4096);

        rprintln!("pre init");

        //
        // Initialize the clocks
        //
        let clocks = rcc
            .cfgr
            .sysclk(80.mhz())
            .lse(CrystalBypass::Enable, ClockSecuritySystem::Disable)
            .freeze(&mut flash.acr, &mut pwr);

        let mono = LowPowerTimer::lptim1(
            lptim,
            LowPowerTimerConfig::default().clock_source(ClockSource::LSE),
            &mut rcc.apb1r1,
            &mut rcc.ccipr,
            clocks,
        );

        rprintln!("init");

        bar::spawn_after(Seconds(1u32)).ok();

        (init::LateResources {}, init::Monotonics(mono.into()))
    }

    #[task]
    fn foo(_: foo::Context) {
        rprintln!("foo");
        bar::spawn_after(Seconds(1u32)).ok();
    }

    #[task]
    fn bar(_: bar::Context) {
        rprintln!("bar");
        foo::spawn_after(Seconds(1u32)).ok();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("idle");

        loop {
            cortex_m::asm::nop();
        }
    }
}
