#![no_std]
#![no_main]

use mcu_hal::Peripherals;
use mcu_rt as _;
use mcu_rt::rtt_log;

const LED_PIN: u16 = 13;
const LED_MASK: u16 = 1 << LED_PIN;

#[unsafe(no_mangle)]
pub fn main() -> ! {
    rtt_log!("mcu_rt booted!");
    rtt_log!("mcu_app start!");

    let dp = unsafe { Peripherals::steal() };
    let gpio = dp.gpio;

    gpio.poerc()
        .modify(|r, w| unsafe { w.bits(r.bits() | LED_MASK) });

    rtt_log!("led initialized!");

    loop {
        gpio.posrc().write(|w| unsafe { w.bits(LED_MASK) });
        rtt_log!("led on!");
        delay(200_000);

        gpio.porrc().write(|w| unsafe { w.bits(LED_MASK) });
        rtt_log!("led off!");
        delay(200_000);
    }
}

#[inline(never)]
fn delay(mut cycles: u32) {
    while cycles != 0 {
        core::hint::spin_loop();
        cycles -= 1;
    }
}
