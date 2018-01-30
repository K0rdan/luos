// This example demonstrates how to read an anlog input.
// The example change the brightness of an led depending of the position of a potentiometer
//
// Board: STM32F072B-DISCO
// Tested on: 16/01/2018

#![no_std]

// luos::hal::servo contains the utils to control a servo
extern crate luos;
use luos::hal::{adc, pwm, rcc, servo};

// servo::Pin contains enums for each pin available for servo on the microcontroller
// on the STM32F072B-DISCO board:

// servo make use internally of pwm functionalities
// servo are controled using a specific pwm frequency (50)
// servo position is controleld via a constrained range of duty cycle
// hence we control a servo on a pwm Pin
// we choose the pin PB4 on the STM32F072B-DISCO
const PIN_SERVO: pwm::Pin = pwm::Pin::PB4;

// define pin for the analog read
// we will use the pin PA0 on the board
const PIN_ANALOG: adc::Pin = adc::Pin::PA0;
// the adc are 12 bits so values range from 0 to 4095 (4096 values)
const MAX_U12: f32 = 4095.0;

// main() is the start of our program
fn main() {
    // initialize rcc
    // it set some register in the microcontroller regarding frequency of timers
    rcc::init();

    // declare `servo` and init it
    let servo = servo::Servo::init(PIN_SERVO);

    // Setup the analog pin for the potentiometer
    let potentiometer = adc::Analog::setup(PIN_ANALOG);

    // as for led_potmeter example we need to convert analog read values, here to degrees
    // analog read return u16,
    // servo.set_position requires f32 in degrees
    fn potentiometer_in_degree(potentiometer_value: u16) -> f32 {
        // we first normalize analog value between 0 and 1, in f32
        let normalized_potentiometer = potentiometer_value as f32 / MAX_U12;
        // we return the value between 0 and 180, in degree
        normalized_potentiometer * 180.0
    }

    // in embedded your program should never end, so we loop forever
    loop {
        // we read the potentiometer
        // transform it into degree
        // and apply it to the servo position
        servo.set_position(potentiometer_in_degree(potentiometer.read()));
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
