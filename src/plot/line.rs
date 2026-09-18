//! A plot that connects a sequence of points with straight segments.

use gpui::{App, Bounds, Hsla, PathBuilder, Pixels, Window, px};

use crate::plot::style::default_color;
use crate::plot::{Plot, map_point};
use crate::point::NormalizedPoint;
use crate::scrub::search::SampleIndex;
use crate::scrub::{ScrubOptions, ScrubSample, nearest, paint_highlight};
use gpui::SharedString;

/// Draws a polyline through its points in the order given.
///
/// Points are expected to be sorted by `x`; the plot does not reorder them.
#[derive(Clone, Debug)]
pub struct LinePlot {
    points: Vec<NormalizedPoint>,
    samples: SampleIndex,
    color: Hsla,
    stroke_width: Pixels,
    labels: Vec<SharedString>,
    scrub: ScrubOptions,
}

impl LinePlot {
    /// Creates a line plot with default styling.
    pub fn new(points: Vec<NormalizedPoint>) -> Self {
        Self {
            samples: SampleIndex::new(&points),
            points,
            color: default_color(),
            stroke_width: px(2.0),
            labels: Vec::new(),
            scrub: ScrubOptions::default(),
        }
    }

    /// Sets the stroke color.
    pub fn with_color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the stroke width.
    pub fn with_stroke_width(mut self, width: Pixels) -> Self {
        self.stroke_width = width;
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

    /// The points connected by this plot.
    pub fn points(&self) -> &[NormalizedPoint] {
        &self.points
    }
}

impl Plot for LinePlot {
    fn paint(&self, area: Bounds<Pixels>, window: &mut Window, _cx: &mut App) {
        let mut in_range = self.points.iter().filter(|p| p.is_in_range());
        let Some(first) = in_range.next() else {
            return;
        };
        let mut builder = PathBuilder::stroke(self.stroke_width);
        builder.move_to(map_point(area, *first));
        for point in in_range {
            builder.line_to(map_point(area, *point));
        }
        if let Ok(path) = builder.build() {
            window.paint_path(path, self.color);
        }
    }

    fn scrub_options(&self) -> ScrubOptions {
        self.scrub
    }

    fn scrub(&self, x: f32) -> Option<ScrubSample> {
        nearest(&self.samples, &self.labels, x)
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
