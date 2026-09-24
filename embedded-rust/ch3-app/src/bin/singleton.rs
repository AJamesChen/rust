#![no_main]
#![no_std]

use ch3_app as _; // global logger + panicking-behavior + memory layout

use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};

struct SerialPort;

struct Peripherals {
    serial: Option<SerialPort>,
}

impl Peripherals {
    const fn new() -> Self {
        Self {
            serial: Some(SerialPort),
        }
    }

    fn take_serial(&mut self) -> Option<SerialPort> {
        self.serial.take()
    }
}

static PERIPHERALS: Mutex<RefCell<Peripherals>> = Mutex::new(RefCell::new(Peripherals::new()));

fn take_serial() -> Option<SerialPort> {
    interrupt::free(|cs| PERIPHERALS.borrow(cs).borrow_mut().take_serial())
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let _serial = take_serial().expect("serial peripheral was already taken");
    defmt::println!("Serial peripheral taken");

    defmt::assert!(take_serial().is_none());
    defmt::println!("A second take was correctly rejected");

    ch3_app::exit()
}
