use units;

/// 2D plan Localizer
pub trait PlanaryLocalizer {
    fn get_position(&self) -> units::Point2D;
}
