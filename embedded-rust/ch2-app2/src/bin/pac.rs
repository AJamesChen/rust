#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3::stm32f303;

// Follow the original comment's intended 528 ticks per PWM period:
// four periods fit in a 2,112-tick video line. The TM4C LOAD=263 value
// counts 0 -> 263 -> 0, which is 526 ticks rather than 528.
const PWM_PERIOD_TICKS: u32 = 2_112 / 4;
const COMPARE_TICKS: u32 = 64;

#[entry]
fn main() -> ! {
    let p = stm32f303::Peripherals::take().unwrap();

    // PA0 is TIM2_CH1 on alternate function 1.
    p.RCC.ahbenr().modify(|_, w| w.iopaen().enabled());
    p.RCC.apb1enr().modify(|_, w| w.tim2en().enabled());
    let _ = p.RCC.apb1enr().read();
    p.GPIOA.afrl().modify(|_, w| w.afrl0().af1());
    p.GPIOA.moder().modify(|_, w| w.moder0().alternate());

    let tim = p.TIM2;
    // PSC=0 uses every TIM2 clock tick. In center-aligned mode the counter
    // traverses 0 -> ARR -> 0, so the period is 2 * ARR ticks.
    tim.psc().write(|w| unsafe { w.psc().bits(0) });
    tim.arr()
        .write(|w| unsafe { w.arr().bits(PWM_PERIOD_TICKS / 2) });
    tim.ccr1().write(|w| unsafe { w.ccr().bits(COMPARE_TICKS) });
    // PWM mode 1 clears the active-high output at compare on the up-count
    // and sets it at compare on the down-count, like the TM4C generator A.
    tim.ccmr1_output().write(|w| w.oc1m().pwm_mode1());
    tim.ccer()
        .write(|w| w.cc1p().rising_edge().cc1e().enabled());
    tim.egr().write(|w| w.ug().set_bit());
    tim.cr1()
        .write(|w| w.cms().center_aligned1().cen().enabled());

    loop {
        cortex_m::asm::wfi();
    }
}
