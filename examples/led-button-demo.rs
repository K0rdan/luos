extern crate luos;

use luos::module::{Button, Led, Module};

const BUTTON_PIN: u8 = 9;
const LED_PIN: u8 = 13;


fn main() {
    let core = luos::Core::new();

    let mut fire_button = Button::new("fire_button", BUTTON_PIN);
    core.register(&fire_button);

    let mut disco_led = Led::new("disco_led", LED_PIN);
    core.register(&disco_led);

    loop {
        // App Update Loop
        if fire_button.pressed() {
            disco_led.on();
        } else {
            disco_led.off();
        }

        // Module and Core Update Loop
        fire_button.update(&core);
        disco_led.update(&core);

        core.update();
    }
}
