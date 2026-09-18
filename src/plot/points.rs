//! A plot that draws a marker at each point.

use gpui::{App, Bounds, Corners, Hsla, Pixels, Window, fill, px};

use crate::plot::style::default_color;
use crate::plot::{Plot, map_point};
use crate::point::NormalizedPoint;
use crate::scrub::{ScrubOptions, ScrubSample, nearest, paint_highlight};
use gpui::SharedString;

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
    labels: Vec<SharedString>,
    scrub: ScrubOptions,
}

impl PointsPlot {
    /// Creates a points plot with default styling.
    pub fn new(points: Vec<NormalizedPoint>) -> Self {
        Self {
            points,
            color: default_color(),
            radius: px(4.0),
            labels: Vec::new(),
            scrub: ScrubOptions::default(),
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

    /// Sets a value label per point, shown when scrubbing.
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.labels = labels.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the scrubber configuration.
    pub fn with_scrub(mut self, scrub: ScrubOptions) -> Self {
        self.scrub = scrub;
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

    fn scrub_options(&self) -> ScrubOptions {
        self.scrub
    }

    fn scrub(&self, x: f32) -> Option<ScrubSample> {
        nearest(&self.points, &self.labels, x)
    }

    fn paint_scrub(
        &self,
        area: Bounds<Pixels>,
        sample: &ScrubSample,
        window: &mut Window,
        cx: &mut App,
    ) {
        paint_highlight(area, sample, self.color, &self.scrub, window, cx);
    }
}
