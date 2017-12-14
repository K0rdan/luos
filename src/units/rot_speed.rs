use core;
use core::num::Float;

/// Used to represent a rotation speed
///
/// It can be used as radians/secondes or degrees/secondes or trun/secondes. Internally, the rotation speed is stored as radians/secondes.
///
/// # Examples
///
/// ```
/// use luos::units::RotSpeed;
/// use luos::units::Angle;
///
/// let alpha = RotSpeed::from_radians_per_sec(Angle::PI.to_radians());
/// assert_eq!(alpha.to_degrees_per_sec(), 180.0);
/// ```
#[derive(Debug)]
pub struct RotSpeed(f32);
impl RotSpeed {
    /// Create an RotSpeed from its value in radians/secondes
    pub fn from_radians_per_sec(r: f32) -> Self {
        RotSpeed(r)
    }
    /// Create an RotSpeed from its value in degrees/secondes
    pub fn from_degrees_per_sec(d: f32) -> Self {
        RotSpeed(d.to_radians())
    }
    /// Create an RotSpeed from its value in turns/secondes
    pub fn from_turns_per_sec(d: f32) -> Self {
        RotSpeed(d * (2.0 * core::f32::consts::PI))
    }
    /// Convert the RotSpeed value to radians/secondes
    pub fn to_radians_per_sec(&self) -> f32 {
        self.0
    }
    /// Convert the RotSpeed value to degrees/secondes
    pub fn to_degrees_per_sec(&self) -> f32 {
        self.0.to_degrees()
    }
    /// Convert the RotSpeed value to turns/secondes
    pub fn to_turns_per_sec(&self) -> f32 {
        self.0 / (2.0 * core::f32::consts::PI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn rot_speed_conversion() {
        let mut rng = rand::thread_rng();
        let d: f32 = Range::new(-720.0, 720.0).ind_sample(&mut rng);

        let a = RotSpeed::from_degrees_per_sec(d);
        let d = RotSpeed::from_radians_per_sec(a.to_radians_per_sec()).to_degrees_per_sec()
            - a.to_degrees_per_sec();
        let error = d.abs();
        assert!(error < core::f32::EPSILON);

        let mut rng = rand::thread_rng();
        let d: f32 = Range::new(-2.0, 2.0).ind_sample(&mut rng);

        let a = RotSpeed::from_turns_per_sec(d);
        let d = RotSpeed::from_radians_per_sec(a.to_radians_per_sec()).to_turns_per_sec()
            - a.to_turns_per_sec();
        let error = d.abs();
        assert!(error < core::f32::EPSILON);

        let mut rng = rand::thread_rng();
        let d: f32 = Range::new(-2.0, 2.0).ind_sample(&mut rng);

        let a = RotSpeed::from_turns_per_sec(d);
        let d = RotSpeed::from_degrees_per_sec(a.to_degrees_per_sec()).to_turns_per_sec()
            - a.to_turns_per_sec();
        let error = d.abs();
        assert!(error < 1e-6);
    }
}
