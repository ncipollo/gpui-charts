//! Builders for assembling graphs, axes, and plots.
//!
//! The raw builders in this module accept normalized coordinates directly and
//! validate that every value lies in `0.0..=1.0`. Friendlier builders that
//! accept timestamps or categories are layered on top of these.

pub mod axes;
pub mod graph;
pub mod plot;

pub use axes::AxesBuilder;
pub use graph::GraphBuilder;
pub use plot::{BarPlotBuilder, LinePlotBuilder, PointsPlotBuilder};

use crate::error::ChartError;
use crate::point::NormalizedPoint;

/// Returns the first out-of-range point as an error.
pub(crate) fn validate_points(points: &[NormalizedPoint]) -> Result<(), ChartError> {
    match points.iter().enumerate().find(|(_, p)| !p.is_in_range()) {
        Some((index, point)) => Err(ChartError::PointOutOfRange {
            index,
            point: *point,
        }),
        None => Ok(()),
    }
}

/// Returns an error if any point has a smaller `x` than its predecessor.
pub(crate) fn validate_sorted(points: &[NormalizedPoint]) -> Result<(), ChartError> {
    match points.windows(2).position(|pair| pair[1].x < pair[0].x) {
        Some(i) => Err(ChartError::UnsortedPoints { index: i + 1 }),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_points, validate_sorted};
    use crate::error::ChartError;
    use crate::point::NormalizedPoint;

    #[test]
    fn reports_first_out_of_range_point() {
        let points = vec![
            NormalizedPoint::new(0.1, 0.1),
            NormalizedPoint::new(1.5, 0.1),
            NormalizedPoint::new(2.0, 0.1),
        ];
        assert_eq!(
            validate_points(&points),
            Err(ChartError::PointOutOfRange {
                index: 1,
                point: NormalizedPoint::new(1.5, 0.1)
            })
        );
    }

    #[test]
    fn reports_first_unsorted_point() {
        let points = vec![
            NormalizedPoint::new(0.1, 0.1),
            NormalizedPoint::new(0.5, 0.1),
            NormalizedPoint::new(0.4, 0.1),
        ];
        assert_eq!(
            validate_sorted(&points),
            Err(ChartError::UnsortedPoints { index: 2 })
        );
        assert_eq!(validate_sorted(&points[..2]), Ok(()));
    }
}
