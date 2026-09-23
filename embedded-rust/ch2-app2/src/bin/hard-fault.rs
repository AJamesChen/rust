#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::ptr;
use cortex_m_rt::{ExceptionFrame, entry, exception};
use defmt_rtt as _;
use panic_probe as _;

// Link the STM32F303 interrupt vector table.
extern crate stm32f3 as _;

#[entry]
fn main() -> ! {
    // read a nonexistent memory location
    #[allow(unsafe_code)]
    unsafe {
        ptr::read_volatile(0x3FFF_0000 as *const u32);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
#[allow(unsafe_code)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    defmt::println!("ExceptionFrame {{");
    defmt::println!("    r0: 0x{=u32:08x},", ef.r0());
    defmt::println!("    r1: 0x{=u32:08x},", ef.r1());
    defmt::println!("    r2: 0x{=u32:08x},", ef.r2());
    defmt::println!("    r3: 0x{=u32:08x},", ef.r3());
    defmt::println!("    r12: 0x{=u32:08x},", ef.r12());
    defmt::println!("    lr: 0x{=u32:08x},", ef.lr());
    defmt::println!("    pc: 0x{=u32:08x},", ef.pc());
    defmt::println!("    xpsr: 0x{=u32:08x},", ef.xpsr());
    defmt::println!("}}");

    loop {
        cortex_m::asm::wfi();
    }
}
