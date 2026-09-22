#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

// Link the STM32F303 interrupt vector table.
extern crate stm32f3;

#[entry]
fn main() -> ! {
    semihosting::println!("Hello, world!");

    loop {
        cortex_m::asm::wfi();
    }
}
