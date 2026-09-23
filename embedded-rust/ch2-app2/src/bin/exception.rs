#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use defmt_rtt as _;
use panic_probe as _;

// Link the STM32F303 interrupt vector table.
extern crate stm32f3 as _;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // Configure SysTick for one interrupt per second. After reset, the
    // STM32F303 runs from the 8 MHz HSI clock.
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000 - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    defmt::println!("SysTick configured for one interrupt per second");

    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    *COUNT += 1;
    defmt::println!("SysTick count: {}", *COUNT);
}
