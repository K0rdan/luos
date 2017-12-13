//! Luos driver - abstract hardware device.

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
