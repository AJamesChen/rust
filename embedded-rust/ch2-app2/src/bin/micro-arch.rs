#![no_std]
#![no_main]

use cortex_m::peripheral::{Peripherals, syst};
use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped() {
        // Loop
    }

    loop {
        cortex_m::asm::wfi();
    }
}
