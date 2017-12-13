//! Standard units used across Luos drivers

use core;
use core::num::Float;

/// Used to represent an Angle
///
/// It can be used as radians or degrees. Internally, the angle is stored as radians.
///
/// # Examples
///
/// ```
/// use luos::units::Angle;
///
/// let alpha = Angle::from_radians(Angle::PI.to_radians());
/// assert_eq!(alpha.to_degrees(), 180.0);
/// ```
#[derive(Debug)]
pub struct Angle(f32);
impl Angle {
    /// Define PI constant
    pub const PI: Angle = Angle(core::f32::consts::PI);

    /// Create an Angle from its value in radians
    pub fn from_radians(r: f32) -> Self {
        Angle(r)
    }
    /// Create an Angle from its value in degrees
    pub fn from_degrees(d: f32) -> Self {
        Angle(d.to_radians())
    }
    /// Convert the angle value to radians
    pub fn to_radians(&self) -> f32 {
        self.0
    }
    /// Convert the angle value to degrees
    pub fn to_degrees(&self) -> f32 {
        self.0.to_degrees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn angle_conversion() {
        let mut rng = rand::thread_rng();
        let d: f32 = Range::new(0.0, 360.0).ind_sample(&mut rng);

        let a = Angle::from_degrees(d);
        let d = Angle::from_radians(a.to_radians()).to_degrees() - a.to_degrees();
        let error = d.abs();
        assert!(error < core::f32::EPSILON);
    }
}
