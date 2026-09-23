#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
// Link the device PAC so cortex-m-rt gets the STM32F303 interrupt vectors.
extern crate stm32f3 as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let xs = [0, 1, 2];
    let i = xs.len();
    let _y = xs[i]; // out of bounds access

    loop {
        cortex_m::asm::wfi();
    }
}
