#![no_main]
#![no_std]

use ch3_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    ch3_app::exit()
}
