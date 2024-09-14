//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::Peripherals as PacPeripherals;
// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, pac::Peripherals as HalPeripherals};

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        run_app(dp, cp);
    }

    loop {}
}

fn run_app(dp: HalPeripherals, cp: PacPeripherals) {
    // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = dp.GPIOC.split();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        if gpioc.pc13.is_high() {
            // On for 1s, off for 1s.
            led.toggle();
        }
        delay.delay_ms(250);
    }
}
