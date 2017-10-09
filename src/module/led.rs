use module::Module;
use luos_core::Core;

use hal;

pub struct Led {
    pin: u8,
}

impl Led {
    pub fn new(pin: u8) -> Led {
        Led { pin }
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
    fn update(&mut self, _core: &Core) {}
}
