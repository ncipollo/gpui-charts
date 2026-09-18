//! Normalized coordinate types shared by every plot and axis.

/// A point in normalized chart space.
///
/// Both `x` and `y` lie in `0.0..=1.0`. `(0, 0)` is the bottom-left corner of
/// the plot area and `(1, 1)` is the top-right corner.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NormalizedPoint {
    /// Horizontal position, `0.0` (left) to `1.0` (right).
    pub x: f32,
    /// Vertical position, `0.0` (bottom) to `1.0` (top).
    pub y: f32,
}

impl NormalizedPoint {
    /// Creates a point from normalized `x` and `y` values.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns `true` when both coordinates lie within `0.0..=1.0`.
    pub fn is_in_range(self) -> bool {
        (0.0..=1.0).contains(&self.x) && (0.0..=1.0).contains(&self.y)
    }
}

impl From<(f32, f32)> for NormalizedPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::NormalizedPoint;

    #[test]
    fn corners_are_in_range() {
        assert!(NormalizedPoint::new(0.0, 0.0).is_in_range());
        assert!(NormalizedPoint::new(1.0, 1.0).is_in_range());
    }

    #[test]
    fn out_of_range_is_detected() {
        assert!(!NormalizedPoint::new(-0.1, 0.5).is_in_range());
        assert!(!NormalizedPoint::new(0.5, 1.1).is_in_range());
    }

    #[test]
    fn converts_from_tuple() {
        assert_eq!(
            NormalizedPoint::from((0.25, 0.75)),
            NormalizedPoint::new(0.25, 0.75)
        );
    }
}
