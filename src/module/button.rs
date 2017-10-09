use module::Module;
use luos_core::Core;

use hal;

pub struct Button {
    pin: u8,
    pressed: bool,
}

impl Button {
    pub fn new(pin: u8) -> Button {
        Button {
            pin,
            pressed: false,
        }
    }
    pub fn pressed(&self) -> bool {
        self.pressed
    }
}

impl Module for Button {
    fn update(&mut self, _core: &Core) {
        self.pressed = hal::digital_read(self.pin) != 0;
    }
}
