#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]


use tmbrctrl as _; // global logger + panicking-behavior + memory layout

use rtic_monotonics::systick::*;

use stm32f1xx_hal::prelude::*;

#[rtic::app(
    device = stm32f1xx_hal::pac,
    // peripherals = true, ??
    dispatchers = [SPI1, SPI2]
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 72_000_000, token);


        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task]
    async fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }
}
