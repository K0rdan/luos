// This example demonstrates how to blink an led on/off at a defined frequency
// It demonstrates how to access low level hardware from luos and use basic time functionalities
//
// Board: STM32F072B-DISCO
// Tested on: 16/01/2018

#![no_std]
/// #![no_std] is needed when compiling for embedded
// We need to specify to the compiler that we don't want to compile the whole standard library
// because it's way too big and would not work on our tiny microcontroller.

// import the luos crate
// luos contains a hal module (hardware abastraction layer)
// luos::hal contains the functions to access peripherals
// luos::hal::gpio contains specific functions and constants to access, read and write on pins
// luos::hal::rcc contains function relative to clocks
extern crate luos;
use luos::hal::{adc, gpio};

// intialize constants for the pin we want to use
// gpio::Pin contains enums for each pin available on the microcontroller
// on the STM32F072B-DISCO board:
// - gpio::Pin::PC7 is a led pin
const PIN_RED_LED: gpio::Pin = gpio::Pin::PC6;
const PIN_BLUE_LED: gpio::Pin = gpio::Pin::PC7;
const PIN_ORANGE_LED: gpio::Pin = gpio::Pin::PC8;
const PIN_GREEN_LED: gpio::Pin = gpio::Pin::PC9;

// http://www.st.com/content/ccc/resource/technical/document/user_manual/3b/8d/46/57/b7/a9/49/b4/DM00099401.pdf/files/DM00099401.pdf/jcr:content/translations/en.DM00099401.pdf
//http://www.st.com/en/evaluation-tools/32f072bdiscovery.html

const PIN_ANALOG: adc::Channel = adc::Channel::ADC0;

// main() is the start of our program
fn main() {
    // declare `led` as an output pin on PIN_LED
    // `led` is mutable as setting the pin to high or low requires to modify it.
    let mut red_led = gpio::Output::setup(PIN_RED_LED);
    let mut blue_led = gpio::Output::setup(PIN_BLUE_LED);
    let mut orange_led = gpio::Output::setup(PIN_ORANGE_LED);
    let mut green_led = gpio::Output::setup(PIN_GREEN_LED);

    let analog = adc::Analog::setup(PIN_ANALOG);

    // in embedded your program should never end, so we loop forever
    loop {
        // The maximum value of this analog is 4096
        if analog.read() > 512 {
            // turn led on -> set pin to high
            red_led.high();
            blue_led.high();
            orange_led.high();
            green_led.high();
        } else {
            // turn led off -> set pin to low
            red_led.low();
            blue_led.low();
            orange_led.low();
            green_led.low();
        }
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
