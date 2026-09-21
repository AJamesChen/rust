#![no_main]
#![no_std]

use ch2_app2 as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    ch2_app2::exit()
}
