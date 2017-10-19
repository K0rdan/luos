//!# Luos: the unity for Robotics
//!
//!Luos is a Rust based embedded OS that let you easily build a robot. You can connect together multiple *Cores*. Each of them can host several *Modules* (both sensor and actuators).
//!
//! The module abstracts the hardware to provide with a standardized API.
//!
//!
//! A simple core containing a LED and a Button module can be written as:
//!
//! ```
//! extern crate luos;
//!
//! use luos::module::{Button, Led, Module};
//! use luos::hal::gpio::Pin;
//!
//! const BUTTON_PIN: Pin = Pin::P9;
//! const LED_PIN: Pin = Pin::P13;
//!
//!
//! fn main() {
//!     let core = luos::Core::new();
//!
//!     let mut button = Button::new("button", BUTTON_PIN);
//!     core.register(&button);
//!
//!     let mut led = Led::new("led", LED_PIN);
//!     core.register(&led);
//!
//!     for _ in 0..10 {
//!         if button.pressed() {
//!             led.on();
//!         } else {
//!             led.off();
//!         }
//!     }
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[cfg(target_arch = "arm")]
extern crate stm32f0_hal;
#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal;

/// Abstract used HAL.
pub mod hal {
    #[cfg(target_arch = "arm")]
    pub use stm32f0_hal::*;
    #[cfg(not(target_arch = "arm"))]
    pub use mockup_hal::*;
}

mod luos_core;
pub use luos_core::Core;

pub mod module;
pub use module::Module;
