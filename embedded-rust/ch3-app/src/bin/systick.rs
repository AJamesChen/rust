#![no_main]
#![no_std]

use ch3_app as _; // global logger + panicking-behavior + memory layout

use volatile_register::{RO, RW};

pub struct SystemTimer {
    p: &'static mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(0xE000_E010 as *mut RegisterBlock) },
        }
    }

    pub fn get_time(&self) -> u32 {
        self.p.cvr.read()
    }

    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe { self.p.rvr.write(reload_value) }
    }
}

pub fn example_usage() -> u32 {
    let mut st = SystemTimer::new();
    st.set_reload(0x00FF_FFFF);
    st.get_time()
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Time is now 0x{=u32:08x}", example_usage());

    loop {
        cortex_m::asm::wfi();
    }
}
