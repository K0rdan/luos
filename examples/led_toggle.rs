// This example demonstrates how to blink an led on/off at a defined frequency
// It demonstrates how to access low level hardware from luos and use basic time functionalities
//
// Board: STM32F072B-DISCO
// Tested on: 16/01/2018

// #![no_std] is needed when compiling for embedded
// We need to specify to the compiler that we don't want to compile the whole standard library
// as it's way too big and would not work on our tiny micro controller.

#![no_std]

// import the luos crate
// luos contains a hal module (hardware abastraction layer)
// luos::hal contains the functions to access peripherals
// luos::hal::gpio contains specific functions and constants to access, read and write on pins
// luos::hal::rcc contains function relative to clocks
extern crate luos;
use luos::hal::{gpio, rcc};

// intialize constants for the pin we want to use
// gpio::Pin contains enums for each pin available on the microprocessor
// on the STM32F072B-DISCO board:
// - gpio::Pin::PC7 is a led pin
const PIN_LED: gpio::Pin = gpio::Pin::PC7;

// main() is the start of our program
fn main() {
    // initialize rcc
    // it set some register in the microcontroller regarding frequency of timers
    rcc::init();

    // declare `led` as an output pin on PIN_LED
    // `led` is mutable: setting the pin to high or low requires to borrow a the variable
    let mut led = gpio::Output::setup(PIN_LED);

    // in embedded your main program loops forever
    loop {
        // turn led on -> set pin to high
        led.high();
        // pause for 1 sec -> 1000 ms
        rcc::ms_delay(1000);
        // turn led off -> set pin to low
        led.low();
        // pause for 1 sec -> 1000 ms
        rcc::ms_delay(1000);
    }
}
