extern crate stm32f0_hal;

use self::stm32f0_hal::gpio;

use module::Module;

/// # Push Bouton Module
/// It provides a pressed method.
///
/// ## Examples
///
/// ```
/// extern crate luos;
///
/// fn main() {
///     let pin = 42;
///     let button = luos::module::Button::new("my_button_alias", pin);
///     if button.pressed() {
///        // do something
///     }
/// }
/// ```
pub struct Button {
    alias: &'static str,
    pin: stm32f0_hal::gpio::Pin,
}

impl Button {
    pub fn new(alias: &'static str, pin: gpio::Pin) -> Button {
        gpio::init(&pin, &gpio::Mode::Input);

        Button { alias, pin }
    }
    pub fn pressed(&self) -> bool {
        gpio::read(&self.pin)
    }
}

impl Module for Button {
    fn alias(&self) -> &'static str {
        self.alias
    }
}
