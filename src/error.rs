//! Errors reported by builders.

use thiserror::Error;

use crate::point::NormalizedPoint;

/// An error produced while validating chart data.
#[derive(Clone, Debug, PartialEq, Error)]
pub enum ChartError {
    /// A point lies outside the normalized `0.0..=1.0` range.
    #[error("point {index} ({x}, {y}) is outside the 0..=1 range", x = point.x, y = point.y)]
    PointOutOfRange {
        /// Index of the offending point in the input.
        index: usize,
        /// The offending point.
        point: NormalizedPoint,
    },
    /// A line plot's points are not sorted by `x`.
    #[error("point {index} has a smaller x than the point before it")]
    UnsortedPoints {
        /// Index of the first point that breaks the ordering.
        index: usize,
    },
    /// An axis label position lies outside `0.0..=1.0`.
    #[error("label {index} at position {position} is outside the 0..=1 range")]
    LabelOutOfRange {
        /// Index of the offending label in the input.
        index: usize,
        /// The offending position.
        position: f32,
    },
}
