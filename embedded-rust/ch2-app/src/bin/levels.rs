#![no_main]
#![no_std]

use ch2_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    // try setting the DEFMT_LOG environment variable
    // e.g. `export DEFMT_LOG=info` or `DEFMT_LOG=trace cargo rb levels`
    defmt::info!("info");
    defmt::trace!("trace");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");

    ch2_app::exit()
}
