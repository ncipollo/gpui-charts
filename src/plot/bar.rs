//! A plot that draws a vertical bar for each point.

use gpui::{App, Bounds, Hsla, Pixels, Window, fill, point, size};

use crate::plot::style::default_color;
use crate::plot::{Plot, map_x, map_y};
use crate::point::NormalizedPoint;
use crate::scrub::search::SampleIndex;
use crate::scrub::{ScrubOptions, ScrubSample, nearest, paint_highlight};
use gpui::SharedString;

/// Draws a bar from the baseline (`y = 0`) up to each point's `y`, centered on
/// the point's `x`.
///
/// Bar width is expressed in normalized `x` units so it scales with the plot
/// area. When no width is set, one is derived from the number of bars.
#[derive(Clone, Debug)]
pub struct BarPlot {
    bars: Vec<NormalizedPoint>,
    samples: SampleIndex,
    color: Hsla,
    width: Option<f32>,
    labels: Vec<SharedString>,
    scrub: ScrubOptions,
}

impl BarPlot {
    /// Creates a bar plot with default styling.
    pub fn new(bars: Vec<NormalizedPoint>) -> Self {
        Self {
            samples: SampleIndex::new(&bars),
            bars,
            color: default_color(),
            width: None,
            labels: Vec::new(),
            scrub: ScrubOptions::default(),
        }
    }

    /// Sets the bar color.
    pub fn with_color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the bar width in normalized `x` units (`0.0..=1.0`).
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets a value label per bar, shown when scrubbing.
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

    /// The bars drawn by this plot, as `(x, height)` points.
    pub fn bars(&self) -> &[NormalizedPoint] {
        &self.bars
    }

    /// The bar width in normalized units, defaulting to 60% of an even slot.
    pub fn width(&self) -> f32 {
        self.width.unwrap_or_else(|| default_width(self.bars.len()))
    }
}

impl Plot for BarPlot {
    fn paint(&self, area: Bounds<Pixels>, window: &mut Window, _cx: &mut App) {
        let width = self.width();
        for bar in self.bars.iter().filter(|b| b.is_in_range()) {
            window.paint_quad(fill(bar_bounds(area, *bar, width), self.color));
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

/// Computes the pixel rectangle for one bar.
fn bar_bounds(area: Bounds<Pixels>, bar: NormalizedPoint, width: f32) -> Bounds<Pixels> {
    let left = map_x(area, bar.x - width / 2.0);
    let right = map_x(area, bar.x + width / 2.0);
    let top = map_y(area, bar.y);
    let bottom = map_y(area, 0.0);
    Bounds::new(point(left, top), size(right - left, bottom - top))
}

/// Derives a bar width that leaves a gap between evenly spaced bars.
fn default_width(count: usize) -> f32 {
    if count == 0 {
        return 0.0;
    }
    0.6 / count as f32
}

#[cfg(test)]
mod tests {
    use gpui::{Bounds, Pixels, point, px, size};

    use super::{bar_bounds, default_width};
    use crate::point::NormalizedPoint;

    fn area() -> Bounds<Pixels> {
        Bounds::new(point(px(0.0), px(0.0)), size(px(100.0), px(100.0)))
    }

    #[test]
    fn bar_spans_from_baseline_to_value() {
        let bounds = bar_bounds(area(), NormalizedPoint::new(0.5, 0.25), 0.2);
        assert_close(bounds.origin.x, px(40.0));
        assert_close(bounds.origin.y, px(75.0));
        assert_close(bounds.size.width, px(20.0));
        assert_close(bounds.size.height, px(25.0));
    }

    fn assert_close(actual: Pixels, expected: Pixels) {
        assert!(
            (actual - expected).abs() < px(0.001),
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[test]
    fn default_width_leaves_gaps() {
        assert_eq!(default_width(0), 0.0);
        assert!((default_width(4) - 0.15).abs() < f32::EPSILON);
    }
}
