use units;

/// Period controler driver
///
/// Must control a period. For instance, a refresh time or a PWM period could satisfy this trait.
///
pub trait PeriodControl {
    /// Set the control period
    fn set_period(&self, period: units::Period);
}
