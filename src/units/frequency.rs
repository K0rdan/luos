/// Used to represent a frequency
///
/// It can be used as second or Hz. Internally, the frequency is stored as Hz.
///
/// # Examples
///
/// ```
/// use luos::units::Frequency;
///
/// let alpha = Frequency::from_hz(1000.0);
/// assert_eq!(alpha.to_seconds(), 0.001);
/// ```
#[derive(Debug)]
pub struct Frequency(f32);
impl Frequency {
    /// Create a frequency from its value in Hz
    pub fn from_hz(r: f32) -> Self {
        Frequency(r)
    }
    /// Create a frequency from its period in seconds
    pub fn from_seconds(d: f32) -> Self {
        if d == 0.0 {
            return Frequency(0.0);
        }
        Frequency(1.0 / d)
    }
    /// Convert the frequency value to Hz
    pub fn to_hz(&self) -> f32 {
        self.0
    }
    /// Convert the frequency value to seconds
    pub fn to_seconds(&self) -> f32 {
        if self.0 == 0.0 {
            return 0.0;
        }
        1.0 / self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn frequency_conversion() {
        let mut rng = rand::thread_rng();
        let d: f32 = Range::new(0.0, 1000.0).ind_sample(&mut rng);

        let a = Frequency::from_seconds(d);
        let d = Frequency::from_hz(a.to_hz()).to_seconds() - a.to_seconds();
        let error = d.abs();
        assert!(error < core::f32::EPSILON);
    }
}
