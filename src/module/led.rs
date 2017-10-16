extern crate stm32f0_hal;

use self::stm32f0_hal::gpio;

use module::Module;

/// # Led Module
/// You can turn the led on and off.
///
/// ## Examples
///
/// ```
/// use std::{thread, time};
///
/// extern crate luos;
///
/// fn main() {
///     let pin = 42;
///     let led = luos::module::Led::new("my_led", pin);
///
///     for _ in 0..5 {
///         led.on();
///         thread::sleep(time::Duration::from_millis(50));
///         led.off();
///         thread::sleep(time::Duration::from_millis(50));
///     }
/// }
/// ```
pub struct Led {
    alias: &'static str,
    pin: gpio::Pin,
}

impl Led {
    pub fn new(alias: &'static str, pin: gpio::Pin) -> Led {
        gpio::init(&pin, &gpio::Mode::Output);

        Led { alias, pin }
    }

    pub fn on(&self) {
        self.set_led(true);
    }

    pub fn off(&self) {
        self.set_led(false);
    }

    fn set_led(&self, on: bool) {
        gpio::write(&self.pin, on);
    }
}

impl Module for Led {
    fn alias(&self) -> &'static str {
        self.alias
    }
}
