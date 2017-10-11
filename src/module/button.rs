use module::Module;

use hal;

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
    pin: u8,
}

impl Button {
    pub fn new(alias: &'static str, pin: u8) -> Button {
        Button { alias, pin }
    }
    pub fn pressed(&self) -> bool {
        hal::digital_read(self.pin) != 0
    }
}

impl Module for Button {
    fn alias(&self) -> &'static str {
        self.alias
    }
}
