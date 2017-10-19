//! Robus module - abstract hardware device.

mod button;
pub use module::button::Button;

mod led;
pub use module::led::Led;

/// # Module trait for all Luos sensor and effector
/// All Luos Module should implement this trait.
///
/// ## Examples
/// A typical implementation of a new module could look like this:
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
/// }
/// ```
pub trait Module {
    fn alias(&self) -> &'static str;
}
