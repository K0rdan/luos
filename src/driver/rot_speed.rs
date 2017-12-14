use units;

/// rotation speed sensor driver
///
/// Must measure the current value of a rotation speed. For instance, a Tachometer could satisfy this trait.
///
pub trait RotSpeedSensor {
    /// Return the current `RotSpeed` measured
    fn get_rot_speed(&self) -> units::RotSpeed;
}
