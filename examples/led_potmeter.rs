// This example demonstrates how to read an anlog input.
// The example change the brightness of an led depending of the position of a potentiometer
//
// Board: STM32F072B-DISCO
// Tested on: 16/01/2018

#![no_std]

// luos::hal::adc contains the function to read from an analog pin
extern crate luos;
use luos::hal::{adc, pwm, rcc};

// define pin for the LED
// PC7 is a led pin and we can set it up as PWM output
const PIN_LED: pwm::Pin = pwm::Pin::PC7;

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

    // declare `led` as an output PWM
    let led = pwm::Pwm::init(PIN_LED);
    // Set the frequency of the PWM to 500 hz
    led.set_frequency(500);
    // Set the duty cycle to 0%
    led.set_duty(0.0);
    // enable the pwm on the pin
    led.enable();

    // Setup the analog pin for the potentiometer
    let potentiometer = adc::Analog::setup(PIN_ANALOG);

    // the set_duty function of pwm takes an argument in percent
    // whereas the analog value are between 0 and 4095
    // thus, we create a function to convert analog values to percents
    // analog read return u16, and set_duty require f32
    fn potentiometer_in_percent(potentiometer_value: u16) -> f32 {
        // we first normalize analog value between 0 and 1, in f32
        let normalized_potentiometer = potentiometer_value as f32 / MAX_U12;
        // we return the value between 0 and 100
        normalized_potentiometer * 100.0
    }

    // in embedded your program should never end, so we loop forever
    loop {
        // we read the potentiometer
        // transform it into percent
        // and apply it to the led pwm
        led.set_duty(potentiometer_in_percent(potentiometer.read()));
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
