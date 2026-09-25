#![no_main]
#![no_std]

use ch3_app as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32f3::stm32f303, peripherals = true)]
mod app {
    use cortex_m::singleton;

    struct SerialPort {
        speed: u32,
    }

    impl SerialPort {
        const fn new(speed: u32) -> Self {
            Self { speed }
        }

        fn read_speed(&self) -> u32 {
            self.speed
        }
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        serial: &'static mut SerialPort,
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let serial = singleton!(SERIAL: SerialPort = SerialPort::new(115_200))
            .expect("SERIAL was already initialized");

        defmt::println!("RTIC initialized the serial singleton");

        (Shared {}, Local { serial }, init::Monotonics())
    }

    #[idle(local = [serial])]
    fn idle(cx: idle::Context) -> ! {
        let speed = cx.local.serial.read_speed();
        defmt::println!("serial speed = {=u32} baud", speed);
        ch3_app::exit()
    }
}
