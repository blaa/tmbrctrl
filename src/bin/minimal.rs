#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]


use tmbrctrl as _; // global logger + panicking-behavior + memory layout

use rtic_monotonics::systick::Systick;

use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    gpio::{self, Output, PushPull}
};

#[rtic::app(
    device = stm32f1xx_hal::pac,
    // peripherals = true, ??
    dispatchers = [SPI1, SPI2]
)]
mod app {
    use super::*;
    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
        led: gpio::PC13<Output<PushPull>>
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        defmt::info!("init");

        let token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 72_000_000, token);

        let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut gpioc = cx.device.GPIOC.split();

        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
                led: gpioc.pc13.into_push_pull_output(&mut gpioc.crh)
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("idle");
        let mut cnt: i64 = 0;

        loop {
            defmt::info!("Idling like hell {}", cnt);
            cnt += 1;
            continue;
        }
    }

    #[task(local=[led], priority=1)]
    async fn task1(cx: task1::Context) {
        defmt::info!("Hello from task1!");
        loop {
            cx.local.led.toggle();
            Systick::delay(50.millis()).await;
            defmt::info!("Hello2 from task1!");
        }
    }
}
