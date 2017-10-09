//! # Luos Module

use luos_core::Core;

mod button;
pub use module::button::Button;

mod led;
pub use module::led::Led;

/// # Module trait for all Luos sensor and effector
/// All Luos Module should implement this trait.
///
/// The main method to implement is the update method. This method is supposed to be called in the the main loop of your core. It's also where you should notify the core of all the changes that has been happeing.
/// ## Examples
/// A typical implementation of the update method could look like this:
///
/// ```
/// extern crate luos;
///
///
/// pub struct Sonar {
///     alias: &'static str,
///     distance: u32,
/// }
///
/// impl luos::module::Module for Sonar {
///     fn alias(&self) -> &'static str {
///         self.alias
///     }
///     fn update(&mut self, _core: &luos::Core) {
///         // In a real example, you would most likely check some PIN.
///         self.distance = 42;
///     }
/// }
/// ```

pub trait Module {
    fn alias(&self) -> &'static str;
    fn update(&mut self, core: &Core);
}
