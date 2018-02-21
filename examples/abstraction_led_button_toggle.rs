// This example starts introducing abstractions
// We will use the button to toggle on/off the led
// This requires additional functionalities for both Led and Button
// The led need to remember its previous state
// And the button need to implement a debounced read method to avoid spurious on/off transitions when pressed, due to mechanical and physical issues
// For that we will create a Led and Button structure with specific method associated to them
//
// Board: STM32F072B-DISCO
// Tested on: 21/02/2018

#![no_std]

extern crate luos;
use luos::hal::{gpio, rcc};

// LED iplementation

// First, we need to create a structure to hold the necessary elements of our augmented Led
pub struct Led {
    pin: gpio::Output, // ouput pin to control the led on and off
    state: bool, // state variable to store the current state of the led (on -> true, off -> false)
}

// Second we associate some methods to this structure
impl Led {
    // Using the new function is a convention to make a sort of constructor
    // It is used to initialize the Led structure with the attached `Pin` and initialize its state.
    // The user needs to provide a GPIO pin and an init_state for that
    pub fn new(pin: gpio::Pin, init_state: bool) -> Led {
        // we create a led_pin variable as an Ouptut pin
        let mut led_pin = gpio::Output::setup(pin);
        // depending on the initial state requested, we set that pin high or low
        match init_state {
            true => led_pin.high(),
            false => led_pin.low(),
        }
        // finally we return a Led structure with those elements all set
        Led {
            pin: led_pin,
            state: init_state,
        }
    }

    // We now implement a toggle method associated to a Led structure
    // It takes as an input a reference of itself as per convention
    // Notice that we use &mut wich indicates that elements of the structure may be changed
    // Indeed the pin state and led state will be affected by toggle
    pub fn toggle(&mut self) {
        // we read the current stored state
        match self.state {
            true => self.pin.low(),   // if true/high -> toggle and set to low
            false => self.pin.high(), // if false/low -> toggle and set to high
        }
        // finally update the current state as the opposite of the previous state
        self.state = !self.state;
    }
}
// At this point we have a structure with associated methods that define an inhenced version of a Led
// This Led can remember its state and toggle from on to off appropriately
// We will see how to initialize this in the main function, but let's see the button before

// Button implementation

// First, we need to create a structure to hold the necessary elements of our augmented Button
pub struct Button {
    pin: gpio::Input,      // input pin to read the button state
    debouce_delay_ms: u32, // this is a fixed timing used to debounced the reading of the button state
}

// Second we associate some methods to this structure
impl Button {
    // We initialize the Button structure with the attached `Pin`, here a GPIO::Pin
    pub fn new(pin: gpio::Pin) -> Button {
        // we return a Button structure
        Button {
            pin: gpio::Input::setup(pin), // with pin holding a gpio input
            debouce_delay_ms: 10,         // and a fixed value for the delay
        }
    }

    // We now implement a debounce_read method associated to a Button structure
    // This debounce read is needed to avoid spurious on/off transitions when pressed due to mechanical issues
    // It leads the programs to read multiple presses in a very short time
    // The method to debounce is to read the value twice in a short period of time
    // If we read twice the same value, we assume thare is not bouncing effect
    // If we don't, then we repeat the process until we get a good reading

    // The debounce_read method does not need to use &mut because no modification to the structure elements is done
    // Note that the function return a bool, that is the state of the button
    pub fn debounce_read(&self) -> bool {
        // loop until we return a value
        loop {
            let first_read = self.pin.read(); // read button state
            rcc::ms_delay(self.debouce_delay_ms); // pause the program for self.debouce_delay_ms (here 10 ms)
            let second_read = self.pin.read(); // read button state again

            // if both reading agrees, we consider there was no bouncing and return the button state
            if first_read == second_read {
                return second_read;
            }
            // else we try again
        }
    }
}

// Let put it all together in the main
fn main() {
    // initialize rcc for the time related functionalities (rcc::ms_delay)
    rcc::init();

    // create a new Led on PC7 with a default state of false/off (the blue led)
    let mut led = Led::new(gpio::Pin::PC7, false);
    // create a new Button on PA0 (the blue user button)
    let button = Button::new(gpio::Pin::PA0);

    // forever
    loop {
        // read the debounced button state
        if button.debounce_read() {
            // toggle the led when the button is pressed
            led.toggle();
            // wait for the button to be released
            while button.debounce_read() {}
        }
    }
    // each time you press the button now, the led with toggle its state
    // this could be done without abstraction but would lead to a much more complex code in the main loop
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
