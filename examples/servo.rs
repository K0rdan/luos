// This example demonstrates how to move a servo motor
// It demonstrates how to access low level hardware from luos and use basic servo
//
// Board: STM32F072B-DISCO
// Tested on: 26/01/2018

#![no_std]
/// #![no_std] is needed when compiling for embedded
// We need to specify to the compiler that we don't want to compile the whole standard library
// because it's way too big and would not work on our tiny microcontroller.

// import the luos crate
// luos contains a hal module (hardware abastraction layer)
// luos::hal contains the functions to access peripherals
// luos::hal::rcc contains function relative to clocks
// luos::hal::adc contains specific functions to manage analogic input pin
// luos::hal::servo contains specific functions to control a servo
extern crate luos;
use luos::hal::{adc, rcc, servo};

// intialize constants for the pin we want to use
// servo::Pin contains enums for each pin available for servo on the microcontroller
// on the STM32F072B-DISCO board:
const PIN_SERVO: servo::Pin = servo::Pin::PB4;
const PIN_ANALOG: adc::Channel = adc::Channel::ADC0; // PA0

// main() is the start of our program
fn main() {
    // initialize rcc
    // it set some register in the microcontroller regarding frequency of timers
    rcc::init();

    // declare `servo` and init it
    let servo = servo::Servo::init(PIN_SERVO);
    // declare `analog` pin
    let analog = adc::Analog::setup(PIN_ANALOG);

    // in embedded your program should never end, so we loop forever
    loop {
        // analog is a 16 bit unsigned value so the maximum is 65535 (0xFFFF)
        // the servo can move from 0° to 180°
        // In this example we want to control the full range of the servo with the full range of the potentiometer
        // So we have to convert the analog value to a degree value
        let degree = ((analog.read() as f32) / 4096 as f32) * 180.0;
        // Now we can set this value
        servo.set_position(degree);
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
