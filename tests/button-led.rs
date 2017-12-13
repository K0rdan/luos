extern crate luos;

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

use luos::driver::Driver;
use hal::gpio;

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
impl Driver for Button {
    fn alias(&self) -> &str {
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
impl Driver for Led {
    fn alias(&self) -> &str {
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
    // to prevent blocking the tests.
    for _ in 0..10 {
        if button.pressed() {
            led.on();
        } else {
            led.off();
        }
    }
}
