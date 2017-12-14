use units;

/// Power controler driver
///
/// Must control the power of a device. For instance, a PWM controled device could satisfy this trait.
///
pub trait PowerControl {
    /// Set the power ratio (-100% to 100%)
    fn set_power(ratio: f32);
    /// Set the control frequency
    fn set_frequency(ratio: units::Frequency);
}
