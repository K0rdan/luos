use units;

/// Frequency controler driver
///
/// Must control a frequency. For instance, a refresh time or a PWM frequency could satisfy this trait.
///
pub trait FrequencyControl {
    /// Set the control frequency
    fn set_frequency(&self, freq: units::Frequency);
}
