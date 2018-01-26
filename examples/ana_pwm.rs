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
use luos::hal::{adc, pwm, rcc};

// intialize constants for the pin we want to use
// pwm::Pin contains enums for each pin available on the microcontroller PWM
// on the STM32F072B-DISCO board, PC6, PC7, PC8, and PC9 are leds pin and we can setup those pin as PWM output

const PIN_RED_LED: pwm::Pin = pwm::Pin::PC6;
const PIN_ORANGE_LED: pwm::Pin = pwm::Pin::PC8;
const PIN_BLUE_LED: pwm::Pin = pwm::Pin::PC7;
const PIN_GREEN_LED: pwm::Pin = pwm::Pin::PC9;

const PIN_ANALOG: adc::Channel = adc::Channel::ADC0;

// main() is the start of our program
fn main() {
    // initialize rcc
    // it set some register in the microcontroller regarding frequency of timers
    rcc::init();

    // Declare `leds` as an output PWM
    let red_led = pwm::Pwm::init(PIN_RED_LED);
    let orange_led = pwm::Pwm::init(PIN_ORANGE_LED);
    let blue_led = pwm::Pwm::init(PIN_BLUE_LED);
    let green_led = pwm::Pwm::init(PIN_GREEN_LED);

    // The PWM frequency is dedicated define the period of the duty cycle to a specific Timer (in this case Timer3)
    // All those led are on the same timer, you have to change this value only one time for all leds
    // So I change it for "red_led" but this frequency is set for all leds
    red_led.set_frequency(500);

    // For now I put the brightness to 0% for all leds
    red_led.set_duty(0.0);
    orange_led.set_duty(0.0);
    blue_led.set_duty(0.0);
    green_led.set_duty(0.0);

    // and I enable PWM for all leds
    red_led.enable();
    orange_led.enable();
    blue_led.enable();
    green_led.enable();

    // Setup the analog pin
    let analog = adc::Analog::setup(PIN_ANALOG);

    // in embedded your program should never end, so we loop forever
    loop {
        // Now I want to have a glorious nightclub effect
        // I want to fade led and give an orientation between those 4 leds using an analog input

        // save analog value
        // analog value is between 0 and 4096
        let direction = analog.read();

        // now I will change the duty cycle depending on the direction variable on each led and by dephasing it
        // to have a cool effect I choose a analog value apogee for each led
        // - red_led => 0
        // - blue_led => 4096/3 = 1365
        // - orange_led => 4096/(2/3) = 2730
        // -green_led => 4096
        if direction < 1365 {
            red_led.set_duty(100.0 - ((direction as f32 / 1365.0) * 100.0));
            orange_led.set_duty((direction as f32 / 1365.0) * 100.0);
            blue_led.set_duty(0.0);
            green_led.set_duty(0.0);
        } else {
            if direction < 2730 {
                red_led.set_duty(0.0);
                orange_led.set_duty(100.0 - (((direction as f32 - 1365.0) / 1365.0) * 100.0));
                blue_led.set_duty(((direction as f32 - 1365.0) / 1365.0) * 100.0);
                green_led.set_duty(0.0);
            } else {
                if direction >= 2730 {
                    red_led.set_duty(0.0);
                    orange_led.set_duty(0.0);
                    blue_led.set_duty(100.0 - (((direction as f32 - 2730.0) / 1365.0) * 100.0));
                    green_led.set_duty(((direction as f32 - 2730.0) / 1365.0) * 100.0);
                }
            }
        }
    }
}

// Is this example not working? have you seen a mistake or a typo?
// You can contribute by raising an issue of the problem
// Or directly submit a pull request to fix this code
