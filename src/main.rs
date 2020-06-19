#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use core::fmt::Write;
use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use hal::{prelude::*, serial::config::Config as SerialConfig, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) =
        (stm32::Peripherals::take(), cortex_m::peripheral::Peripherals::take())
    {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let serial_config = SerialConfig::default().baudrate(9600.bps());
        let usart1 = dp.USART1;

        let gpioa = dp.GPIOA.split();
        let usart_pins = (gpioa.pa9.into_alternate_af7(), gpioa.pa10.into_alternate_af7());
        let usart = hal::serial::Serial::usart1(usart1, usart_pins, serial_config, clocks).unwrap();
        let (mut tx, _rx) = usart.split();

        loop {
            // On for 1s, off for 1s.
            led.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            delay.delay_ms(1000_u32);
            writeln!(tx, "Hello\r").unwrap();
        }
    }

    loop {}
}
