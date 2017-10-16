#![feature(used)]
#![no_std]

extern crate luos;
extern crate stm32f0_hal;

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;

use luos::module::{Button, Led};
use stm32f0_hal::gpio::Pin;

const BUTTON_PIN: Pin = Pin::P6;
const LED_PIN: Pin = Pin::P10;


fn main() {
    let core = luos::Core::new();

    let fire_button = Button::new("fire_button", BUTTON_PIN);
    core.register(&fire_button);

    let disco_led = Led::new("disco_led", LED_PIN);
    core.register(&disco_led);

    loop {
        if fire_button.pressed() {
            disco_led.on();
        } else {
            disco_led.off();
        }
    }
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
