#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_halt as _;
use stm32f401_pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    dp.rcc
        .ahb1enr()
        .write(|w| w.gpioaen().set_bit().gpiocen().set_bit());
    dp.gpioa.moder().write(|w| unsafe { w.moder5().bits(0b01) });
    loop {
        // Read PC13 Input Value
        if !dp.gpioc.idr().read().idr13().bit() {
            dp.gpioa.odr().write(|w| w.odr5().set_bit());
        } else {
            dp.gpioa.odr().write(|w| w.odr5().clear_bit());
        }
    }
}
