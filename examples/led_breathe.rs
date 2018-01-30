// This example demonstrates how to use the pwm functionalities and use it to change the brightness of the led
//
// Board: STM32F072B-DISCO
// Tested on: 30/01/2018

#![no_std]

// luos::hal::pwm contains function relative to pwm
// we also need luos::hal::rcc that handle and setup the clocks in the microcontroller
extern crate luos;
use luos::hal::{pwm, rcc};

// initialize constants for the pin we want to use
// pwm::Pin contains enums for each pin available on the microcontroller PWM
// on the STM32F072B-DISCO board, PC7 is a led pin and we can set it up as PWM output
const PIN_LED: pwm::Pin = pwm::Pin::PC7;
// compared to the led_toggle example, the pin is the same PC7 but we access it via the pwm module
// this will be changed in the furture for a more consistent API

fn main() {
    // initialize rcc
    // it set some register in the microcontroller regarding frequency of timers
    rcc::init();

    // declare `led` as an output PWM
    let led = pwm::Pwm::init(PIN_LED);
    // We need to set the frequency of the pwm
    // Here we arbitrarily choose to generate a pwm at 500hz
    led.set_frequency(500);
    // Set the duty cycle to 0%
    led.set_duty(0.0);
    // enable the pwm on the pin
    led.enable();

    // in embedded your program should never end, so we loop forever
    loop {
        // we want to have a breathing led
        // to do so we want to repeatidly increase and decrease the brightness of the led
        // this is done by changing the duty cycle of the PWM from 0 to 100 % at fixed time intervals

        // first increase brightness from 0 to 100 percent
        for percent in 0..101 {
            led.set_duty(percent as f32); // set_duty requires a f32
            rcc::ms_delay(10); //pause for 10 ms
        }
        // then decrease brightness from 100 to 0 percent
        // .rev() on the range
        for percent in (0..101).rev() {
            led.set_duty(percent as f32); // set_duty requires a f32
            rcc::ms_delay(10); //pause for 10 ms
        }
        // repeat
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
