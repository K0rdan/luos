extern crate luos;

use luos::Module;
use luos::hal::gpio;

struct Button {
    alias: &'static str,
    pin: gpio::Input,
}
impl Button {
    pub fn new(alias: &'static str, pin: gpio::Pin) -> Button {
        Button {
            alias,
            pin: gpio::Input::setup(pin),
        }
    }
    pub fn pressed(&self) -> bool {
        self.pin.read()
    }
}
impl Module for Button {
    fn alias(&self) -> &'static str {
        self.alias
    }
}

struct Led {
    alias: &'static str,
    pin: gpio::Output,
}
impl Led {
    pub fn new(alias: &'static str, pin: gpio::Pin) -> Led {
        Led {
            alias,
            pin: gpio::Output::setup(pin),
        }
    }
    pub fn on(&mut self) {
        self.pin.high();
    }
    pub fn off(&mut self) {
        self.pin.low();
    }
}
impl Module for Led {
    fn alias(&self) -> &'static str {
        self.alias
    }
}

#[test]
fn register_and_run() {
    let core = luos::Core::new();

    let button = Button::new("fire_button", gpio::Pin::PA1);
    core.register(&button);

    let mut led = Led::new("disco_led", gpio::Pin::PA5);
    core.register(&led);

    // In a normal example, we will most likely
    // use an infinite loop there.
    // Yet, in this test we simply make few loop iterations
    // to prevent blocking testing.
    for _ in 0..10 {
        if button.pressed() {
            led.on();
        } else {
            led.off();
        }
    }
}
