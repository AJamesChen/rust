#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3::stm32f303;

// USART1 uses the 8 MHz HSI clock, independent of the system PLL.
const USART_CLOCK_HZ: u32 = 8_000_000;
const BAUD: u32 = 115_200;
const BRR: u16 = ((USART_CLOCK_HZ + BAUD / 2) / BAUD) as u16;

#[entry]
fn main() -> ! {
    let p = stm32f303::Peripherals::take().unwrap();

    // On STM32F3DISCOVERY revision C and newer, PC4/PC5 connect USART1
    // TX/RX to the ST-LINK virtual COM port through SB13/SB15.
    p.RCC.ahbenr().modify(|_, w| w.iopcen().enabled());
    p.RCC.apb2enr().modify(|_, w| w.usart1en().enabled());
    p.RCC.cfgr3().modify(|_, w| w.usart1sw().hsi());
    let _ = p.RCC.apb2enr().read();

    p.GPIOC.afrl().modify(|_, w| w.afrl4().af7().afrl5().af7());
    p.GPIOC
        .moder()
        .modify(|_, w| w.moder4().alternate().moder5().alternate());

    let uart = p.USART1;
    // BRR = round(8 MHz / 115200) with the reset default of 16x oversampling.
    uart.brr().write(|w| unsafe { w.brr().bits(BRR) });
    uart.cr1()
        .write(|w| w.te().enabled().re().enabled().ue().enabled());
    while uart.isr().read().teack().bit_is_clear() {}

    loop {
        for &byte in b"Hello, World!\r\n" {
            while uart.isr().read().txe().bit_is_clear() {}
            uart.tdr()
                .write(|w| unsafe { w.tdr().bits(u16::from(byte)) });
        }
    }
}
