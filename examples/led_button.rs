// This example demonstrate how to turn an led on/off depending on the state of a button.
// It demonstrate how to access low level hardware from luos.
//
// Board: STM32F072B-DISCO
// Tested on: 16/01/2018

#![no_std]
/// #![no_std] is needed when compiling for embedded
// We need to specify to the compiler that we don't want to compile the whole standard library as it's way too big and would not work on our tiny micro controller.

// import the luos crate
// luos contains a hal module (hardware abastraction layer)
// luos::hal contains the functions to access peripherals
// luos::hal::gpio contains specific functions and constants to access, read and write on pins
extern crate luos;
use luos::hal::gpio;

// intialize constants for each pin we want to use
// gpio::Pin contains enums for each pin available on the microcontroler
// on the STM32F072B-DISCO board:
// - gpio::Pin::PC7 is a led pin
// - gpio::Pin::PA0 is the user button (the blue one on the board)
const PIN_LED: gpio::Pin = gpio::Pin::PC7;
const PIN_BUTTON: gpio::Pin = gpio::Pin::PA0;

// main() is the start of our program
fn main() {
    // declare `led` as an output pin on PIN_LED
    // `led` is mutable: setting the pin to high or low requires to borrow the variable
    let mut led = gpio::Output::setup(PIN_LED);

    // declare `button` as an input pin on PIN_BUTTON
    let button = gpio::Input::setup(PIN_BUTTON);

    // in embedded your main program use 100% of the CPU so you have to loops forever
    loop {
        // read state of the button
        // the read function returns a boolean: True or False
        if button.read() {
            // if button is pressed
            // turn led on -> set pin to high
            led.high();
        } else {
            // is button is released
            // turn led off -> set pin to low
            led.low();
        }
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
