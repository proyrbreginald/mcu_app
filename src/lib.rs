#![no_std]

use mcu_hal::Peripherals;

const LED_PIN: u16 = 13;
const LED_MASK: u16 = 1 << LED_PIN;

#[unsafe(no_mangle)]
pub fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };
    let gpio = dp.gpio;

    gpio.poerc()
        .modify(|r, w| unsafe { w.bits(r.bits() | LED_MASK) });

    loop {
        gpio.posrc().write(|w| unsafe { w.bits(LED_MASK) });
        delay(200_000);

        gpio.porrc().write(|w| unsafe { w.bits(LED_MASK) });
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
