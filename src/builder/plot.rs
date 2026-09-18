//! Builders for the concrete plot types.

use gpui::{Hsla, Pixels};

use crate::builder::{validate_points, validate_sorted};
use crate::error::ChartError;
use crate::plot::bar::BarPlot;
use crate::plot::line::LinePlot;
use crate::plot::points::PointsPlot;
use crate::point::NormalizedPoint;

/// Builds a [`PointsPlot`] from normalized coordinates.
#[derive(Debug, Default)]
pub struct PointsPlotBuilder {
    points: Vec<NormalizedPoint>,
    color: Option<Hsla>,
    radius: Option<Pixels>,
}

impl PointsPlotBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one point.
    pub fn point(mut self, x: f32, y: f32) -> Self {
        self.points.push(NormalizedPoint::new(x, y));
        self
    }

    /// Appends many points.
    pub fn points(mut self, points: impl IntoIterator<Item = impl Into<NormalizedPoint>>) -> Self {
        self.points.extend(points.into_iter().map(Into::into));
        self
    }

    /// Sets the marker color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the marker radius.
    pub fn radius(mut self, radius: Pixels) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Validates the points and builds the plot.
    pub fn build(self) -> Result<PointsPlot, ChartError> {
        validate_points(&self.points)?;
        let mut plot = PointsPlot::new(self.points);
        if let Some(color) = self.color {
            plot = plot.with_color(color);
        }
        if let Some(radius) = self.radius {
            plot = plot.with_radius(radius);
        }
        Ok(plot)
    }
}

/// Builds a [`LinePlot`] from normalized coordinates sorted by `x`.
#[derive(Debug, Default)]
pub struct LinePlotBuilder {
    points: Vec<NormalizedPoint>,
    color: Option<Hsla>,
    stroke_width: Option<Pixels>,
}

impl LinePlotBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one point.
    pub fn point(mut self, x: f32, y: f32) -> Self {
        self.points.push(NormalizedPoint::new(x, y));
        self
    }

    /// Appends many points.
    pub fn points(mut self, points: impl IntoIterator<Item = impl Into<NormalizedPoint>>) -> Self {
        self.points.extend(points.into_iter().map(Into::into));
        self
    }

    /// Sets the stroke color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the stroke width.
    pub fn stroke_width(mut self, width: Pixels) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Validates the points (range and `x` ordering) and builds the plot.
    pub fn build(self) -> Result<LinePlot, ChartError> {
        validate_points(&self.points)?;
        validate_sorted(&self.points)?;
        let mut plot = LinePlot::new(self.points);
        if let Some(color) = self.color {
            plot = plot.with_color(color);
        }
        if let Some(width) = self.stroke_width {
            plot = plot.with_stroke_width(width);
        }
        Ok(plot)
    }
}

/// Builds a [`BarPlot`] from normalized `(x, height)` pairs.
#[derive(Debug, Default)]
pub struct BarPlotBuilder {
    bars: Vec<NormalizedPoint>,
    color: Option<Hsla>,
    width: Option<f32>,
}

impl BarPlotBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one bar centered at `x` with normalized height `y`.
    pub fn bar(mut self, x: f32, y: f32) -> Self {
        self.bars.push(NormalizedPoint::new(x, y));
        self
    }

    /// Appends many bars.
    pub fn bars(mut self, bars: impl IntoIterator<Item = impl Into<NormalizedPoint>>) -> Self {
        self.bars.extend(bars.into_iter().map(Into::into));
        self
    }

    /// Sets the bar color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the bar width in normalized `x` units.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Validates the bars and builds the plot.
    pub fn build(self) -> Result<BarPlot, ChartError> {
        validate_points(&self.bars)?;
        let mut plot = BarPlot::new(self.bars);
        if let Some(color) = self.color {
            plot = plot.with_color(color);
        }
        if let Some(width) = self.width {
            plot = plot.with_width(width);
        }
        Ok(plot)
    }
}

#[cfg(test)]
mod tests {
    use super::{BarPlotBuilder, LinePlotBuilder, PointsPlotBuilder};
    use crate::error::ChartError;
    use crate::point::NormalizedPoint;

    #[test]
    fn points_builder_collects_points() {
        let plot = PointsPlotBuilder::new()
            .point(0.1, 0.2)
            .points([(0.3, 0.4)])
            .build()
            .expect("valid points");
        assert_eq!(
            plot.points(),
            &[
                NormalizedPoint::new(0.1, 0.2),
                NormalizedPoint::new(0.3, 0.4)
            ]
        );
    }

    #[test]
    fn points_builder_rejects_out_of_range() {
        let err = PointsPlotBuilder::new()
            .point(0.5, -0.1)
            .build()
            .unwrap_err();
        assert!(matches!(err, ChartError::PointOutOfRange { index: 0, .. }));
    }

    #[test]
    fn line_builder_rejects_unsorted() {
        let err = LinePlotBuilder::new()
            .point(0.5, 0.5)
            .point(0.2, 0.5)
            .build()
            .unwrap_err();
        assert_eq!(err, ChartError::UnsortedPoints { index: 1 });
    }

    #[test]
    fn bar_builder_applies_width() {
        let plot = BarPlotBuilder::new()
            .bar(0.5, 0.5)
            .width(0.25)
            .build()
            .expect("valid bars");
        assert!((plot.width() - 0.25).abs() < f32::EPSILON);
    }
}
