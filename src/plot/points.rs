//! A plot that draws a marker at each point.

use gpui::{App, Bounds, Corners, Hsla, Pixels, Window, fill, px};

use crate::plot::style::default_color;
use crate::plot::{Plot, map_point};
use crate::point::NormalizedPoint;

/// Draws a circular marker at each normalized point.
///
/// Use it on its own for a scatter chart or layer it over a [`LinePlot`] to
/// emphasise the samples.
///
/// [`LinePlot`]: crate::plot::line::LinePlot
#[derive(Clone, Debug)]
pub struct PointsPlot {
    points: Vec<NormalizedPoint>,
    color: Hsla,
    radius: Pixels,
}

impl PointsPlot {
    /// Creates a points plot with default styling.
    pub fn new(points: Vec<NormalizedPoint>) -> Self {
        Self {
            points,
            color: default_color(),
            radius: px(4.0),
        }
    }

    /// Sets the marker color.
    pub fn with_color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the marker radius.
    pub fn with_radius(mut self, radius: Pixels) -> Self {
        self.radius = radius;
        self
    }

    /// The points drawn by this plot.
    pub fn points(&self) -> &[NormalizedPoint] {
        &self.points
    }
}

impl Plot for PointsPlot {
    fn paint(&self, area: Bounds<Pixels>, window: &mut Window, _cx: &mut App) {
        let diameter = self.radius * 2.0;
        for point in self.points.iter().filter(|p| p.is_in_range()) {
            let bounds =
                Bounds::centered_at(map_point(area, *point), gpui::size(diameter, diameter));
            let mut quad = fill(bounds, self.color);
            quad.corner_radii = Corners::all(self.radius);
            window.paint_quad(quad);
        }
    }
}
