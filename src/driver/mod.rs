//! Luos driver - abstract hardware device.
//!
//! A driver purpose is abstracting hardware to expose a standardized API. Driver are thus designed to be modular and can be easily combined with each others, replaced by a similar one, etc.
//!
//! A driver is made to be sandboxed and can only access its internal state.

mod angle;
pub use self::angle::AngleSensor;

/// Driver trait for all Luos sensor and effector
///
/// All Luos Driver should implement this trait.
///
/// ## Examples
/// A typical implementation of a new driver could look like this:
///
/// ```
/// extern crate luos;
///
/// pub struct Sonar {
///     alias: &'static str,
///     distance: u32,
/// }
///
/// impl luos::driver::Driver for Sonar {
///     fn alias(&self) -> &'static str {
///         self.alias
///     }
/// }
/// ```
pub trait Driver {
    fn alias(&self) -> &str;
}
