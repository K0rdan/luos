use module::Module;
use luos_core::Core;

use hal;

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
    pin: u8,
}

impl Led {
    pub fn new(alias: &'static str, pin: u8) -> Led {
        Led { alias, pin }
    }

    pub fn on(&self) {
        self.set_led(true);
    }

    pub fn off(&self) {
        self.set_led(false);
    }

    fn set_led(&self, on: bool) {
        hal::digital_write(self.pin, on as u8);
    }
}

impl Module for Led {
    fn alias(&self) -> &'static str {
        self.alias
    }
    fn update(&mut self, _core: &Core) {}
}
