//! The [`Plot`] abstraction: one implementation per kind of mark.
//!
//! Concrete plots (points, line, bar) live in submodules of this module.
//! Highlights and overlays are expressed as plots too, so a [`Graph`] only ever
//! deals with a list of `dyn Plot`.
//!
//! [`Graph`]: crate::Graph

pub mod bar;
pub mod line;
pub mod points;
pub mod style;

use gpui::{App, Bounds, Pixels, Point, Window, point};

use crate::plot::style::default_color;
use crate::point::NormalizedPoint;
use crate::scrub::{ScrubOptions, ScrubSample, paint_highlight};

/// A drawable layer of a graph.
///
/// Plots paint directly with gpui's low-level paint API. The graph calls
/// [`Plot::paint`] once per frame from inside a canvas paint callback, passing
/// the pixel bounds of the plot area (the region inside the axes). Plots map
/// their normalized coordinates into that area with [`map_point`] and must not
/// draw outside it.
///
/// The trait is object-safe so a graph can hold `Vec<Box<dyn Plot>>`.
pub trait Plot {
    /// Paints this plot into `area`.
    fn paint(&self, area: Bounds<Pixels>, window: &mut Window, cx: &mut App);

    /// This plot's scrubber configuration. Scrubbing is off by default.
    fn scrub_options(&self) -> ScrubOptions {
        ScrubOptions::default()
    }

    /// Returns the sample nearest normalized `x`, or `None` when this plot has
    /// nothing to scrub. The graph only calls this when
    /// [`Plot::scrub_options`] says the scrubber is triggered.
    fn scrub(&self, _x: f32) -> Option<ScrubSample> {
        None
    }

    /// Paints the highlight for a scrubbed sample. The default draws a guide
    /// line, a ring around the point, and the value when `show_value` is set.
    fn paint_scrub(
        &self,
        area: Bounds<Pixels>,
        sample: &ScrubSample,
        window: &mut Window,
        cx: &mut App,
    ) {
        let options = self.scrub_options();
        paint_highlight(area, sample, default_color(), &options, window, cx);
    }
}

/// Maps a normalized point into pixel space within `area`.
///
/// Normalized `(0, 0)` is the bottom-left corner of `area` and `(1, 1)` is the
/// top-right. gpui's y axis points down, so `y` is flipped.
pub fn map_point(area: Bounds<Pixels>, p: NormalizedPoint) -> Point<Pixels> {
    point(map_x(area, p.x), map_y(area, p.y))
}

/// Maps a normalized `x` (0 = left, 1 = right) into pixel space within `area`.
pub fn map_x(area: Bounds<Pixels>, x: f32) -> Pixels {
    area.origin.x + area.size.width * x
}

/// Maps a normalized `y` (0 = bottom, 1 = top) into pixel space within `area`.
pub fn map_y(area: Bounds<Pixels>, y: f32) -> Pixels {
    area.origin.y + area.size.height * (1.0 - y)
}

#[cfg(test)]
mod tests {
    use gpui::{Bounds, Pixels, point, px, size};

    use super::map_point;
    use crate::point::NormalizedPoint;

    fn area() -> Bounds<Pixels> {
        Bounds::new(point(px(10.0), px(20.0)), size(px(100.0), px(50.0)))
    }

    #[test]
    fn origin_maps_to_bottom_left() {
        let mapped = map_point(area(), NormalizedPoint::new(0.0, 0.0));
        assert_eq!(mapped, point(px(10.0), px(70.0)));
    }

    #[test]
    fn unit_maps_to_top_right() {
        let mapped = map_point(area(), NormalizedPoint::new(1.0, 1.0));
        assert_eq!(mapped, point(px(110.0), px(20.0)));
    }

    #[test]
    fn midpoint_maps_to_center() {
        let mapped = map_point(area(), NormalizedPoint::new(0.5, 0.5));
        assert_eq!(mapped, point(px(60.0), px(45.0)));
    }
}
