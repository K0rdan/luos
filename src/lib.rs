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
//!
//! const BUTTON_PIN: u8 = 9;
//! const LED_PIN: u8 = 13;
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

mod luos_core;
pub use luos_core::Core;

pub mod module;
