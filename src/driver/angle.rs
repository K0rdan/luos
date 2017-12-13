use units;

/// Angle sensor driver
///
/// Must measure the current value of an angle. For instance, a wheel encoder could satisfy this trait.
///
pub trait AngleSensor {
    /// Return the current `Angle` measured
    fn get_angle(&self) -> units::Angle;
}
