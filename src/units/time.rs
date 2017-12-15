#[cfg(target_arch = "arm")]
use core::num::Float;

/// Used to represent a Frequency
///
/// Internally, the frequency is stored as Hz. It can be converted into a Period.
///
/// # Examples
///
/// ```
/// use luos::units::{Period, Frequency};
///
/// let f = Frequency::from_hz(1000.0);
///
/// let hz = f.as_hz();
/// let period: Period = f.into();
/// assert_eq!(period.as_seconds(), 1.0 / hz);
/// ```
#[derive(Debug)]
pub struct Frequency(f32);
impl Frequency {
    /// Create a Frequency from its value in Hz
    pub fn from_hz(f: f32) -> Self {
        assert!(f.is_sign_positive());

        Frequency(f)
    }
    /// Get the Frequency value as Hz
    pub fn as_hz(&self) -> f32 {
        self.0
    }
}
/// Used to represent a Period
///
/// Internally, the period is stored in seconds. It can be converted into a Frequency.
///
/// # Examples
///
/// ```
/// use luos::units::{Period, Frequency};
///
/// let p = Period::from_seconds(0.1);
///
/// let s = p.as_seconds();
/// let f: Frequency = p.into();
/// assert_eq!(s, 1.0 / f.as_hz());
/// ```
#[derive(Debug)]
pub struct Period(f32);
impl Period {
    /// Create a Period from its value in seconds
    pub fn from_seconds(s: f32) -> Self {
        assert!(s.is_sign_positive());
        Period(s)
    }
    /// Get the Period value in seconds
    pub fn as_seconds(&self) -> f32 {
        self.0
    }
    /// Get the Period value in milliseconds
    pub fn as_millis(&self) -> f32 {
        self.0 * 1000.0
    }
    /// Get the Period value in microsecond
    pub fn as_micros(&self) -> f32 {
        self.0 * 1000000.0
    }
}

impl Into<Period> for Frequency {
    fn into(self) -> Period {
        Period::from_seconds(1.0 / self.as_hz())
    }
}

impl Into<Frequency> for Period {
    fn into(self) -> Frequency {
        Frequency::from_hz(1.0 / self.as_seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn freq_to_period() {
        let mut rng = rand::thread_rng();
        let s: f32 = Range::new(0.0, 1000.0).ind_sample(&mut rng);

        let p = Period::from_seconds(s);
        let f = p.into::<Frequency>();
        let p: Period = f.into();

        assert!((p.as_seconds() - s).abs() < 1e-4);
    }

    #[test]
    fn frequency_conversion() {
        let mut rng = rand::thread_rng();
        let f: f32 = Range::new(0.0, 1000.0).ind_sample(&mut rng);

        assert_eq!(Frequency::from_hz(f).as_hz(), f);
    }
    #[test]
    #[should_panic]
    fn negative_frequency() {
        let mut rng = rand::thread_rng();
        let f: f32 = Range::new(-1000.0, 0.0).ind_sample(&mut rng);
        Frequency::from_hz(f);
    }
    #[test]
    fn period_conversion() {
        let mut rng = rand::thread_rng();
        let s: f32 = Range::new(0.0, 1000.0).ind_sample(&mut rng);

        let p = Period::from_seconds(s);
        assert_eq!(p.as_seconds(), s);
        assert_eq!(p.as_millis(), s * 1000.0);
    }
    #[test]
    #[should_panic]
    fn negative_period() {
        let mut rng = rand::thread_rng();
        let s: f32 = Range::new(-1000.0, 0.0).ind_sample(&mut rng);
        Period::from_seconds(s);
    }
}
