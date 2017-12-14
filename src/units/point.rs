/// Represent a 2D point (x, y)
///
/// # Examples
///
/// ```
/// use luos::units::Point2D;
///
/// let pt = Point2D::origin();
///
/// assert_eq!(pt.x, 0.0);
/// assert_eq!(pt.y, 0.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}
impl Point2D {
    pub fn new(x: f32, y: f32) -> Point2D {
        Point2D { x, y }
    }
    pub fn origin() -> Point2D {
        Point2D::new(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn create_point() {
        let mut rng = rand::thread_rng();

        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        let pt = Point2D::new(x, y);

        assert_eq!(pt.x, x);
        assert_eq!(pt.y, y);
    }
}
