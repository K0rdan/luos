use module::Module;
use luos_core::Core;

use hal;

pub struct Button {
    alias: &'static str,
    pin: u8,
    pressed: bool,
}

impl Button {
    pub fn new(alias: &'static str, pin: u8) -> Button {
        Button {
            alias,
            pin,
            pressed: false,
        }
    }
    pub fn pressed(&self) -> bool {
        self.pressed
    }
}

impl Module for Button {
    fn alias(&self) -> &'static str {
        self.alias
    }
    fn update(&mut self, _core: &Core) {
        self.pressed = hal::digital_read(self.pin) != 0;
    }
}
