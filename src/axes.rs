//! Horizontal and vertical axes with their labels.
//!
//! Axes are purely positional: each label sits at a normalized position along
//! its axis. Higher-level builders decide what the labels say.

pub mod horizontal;
pub mod label;
pub mod style;
pub mod vertical;

use gpui::{App, Bounds, Edges, Pixels, SharedString, Window, point, px, size};

use crate::axes::horizontal::HorizontalAxis;
use crate::axes::vertical::VerticalAxis;

pub use label::AxisLabel;
pub use style::{AxisStyle, measure_label};

/// Extra room kept around the frame so label edges never touch the element edge.
const LABEL_SLACK: Pixels = px(2.0);

/// The axes rendered around a graph's plot area.
///
/// Either axis may be omitted, in which case no gutter is reserved for it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Axes {
    horizontal: Option<HorizontalAxis>,
    vertical: Option<VerticalAxis>,
}

impl Axes {
    /// Creates a set of axes from optional horizontal and vertical axes.
    pub fn new(horizontal: Option<HorizontalAxis>, vertical: Option<VerticalAxis>) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    /// The horizontal (bottom) axis, if any.
    pub fn horizontal(&self) -> Option<&HorizontalAxis> {
        self.horizontal.as_ref()
    }

    /// The vertical (left) axis, if any.
    pub fn vertical(&self) -> Option<&VerticalAxis> {
        self.vertical.as_ref()
    }

    /// Computes the frame left over after reserving room for the axes.
    ///
    /// The vertical axis reserves a gutter on the left and the horizontal
    /// axis one along the bottom. Labels near the ends of an axis are
    /// centered on their tick, so the frame is also pulled in wherever a
    /// label would otherwise spill past `bounds`. `measure` returns the
    /// painted width of a label's text.
    pub fn frame(
        &self,
        bounds: Bounds<Pixels>,
        measure: impl Fn(&SharedString, &AxisStyle) -> Pixels,
    ) -> Bounds<Pixels> {
        let insets = self.insets(bounds, measure);
        let width = (bounds.size.width - insets.left - insets.right).max(Pixels::ZERO);
        let height = (bounds.size.height - insets.top - insets.bottom).max(Pixels::ZERO);
        Bounds::new(
            point(bounds.origin.x + insets.left, bounds.origin.y + insets.top),
            size(width, height),
        )
    }

    fn insets(
        &self,
        bounds: Bounds<Pixels>,
        measure: impl Fn(&SharedString, &AxisStyle) -> Pixels,
    ) -> Edges<Pixels> {
        let mut insets = Edges::default();
        if let Some(axis) = &self.vertical {
            insets.left = axis.style().gutter;
            let half_line = axis.style().line_height() / 2.0;
            let (top, bottom) =
                end_overhang(axis.labels(), |_| half_line, bounds.size.height, true);
            insets.top = top;
            insets.bottom = bottom;
        }
        if let Some(axis) = &self.horizontal {
            insets.bottom = insets.bottom.max(axis.style().gutter);
            let width = bounds.size.width - insets.left;
            let half_width = |label: &AxisLabel| measure(&label.text, axis.style()) / 2.0;
            let (left, right) = end_overhang(axis.labels(), half_width, width, false);
            insets.left = insets.left.max(left);
            insets.right = right;
        }
        insets
    }

    /// Paints both axes. Baselines span `frame`; ticks and labels line up with
    /// the normalized positions inside `area`.
    pub fn paint(
        &self,
        frame: Bounds<Pixels>,
        area: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(axis) = &self.horizontal {
            axis.paint(frame, area, window, cx);
        }
        if let Some(axis) = &self.vertical {
            axis.paint(frame, area, window, cx);
        }
    }
}

/// How far labels centered on their ticks spill past each end of an axis of
/// length `extent`. Returns `(start, end)` overhang; for a flipped axis the
/// start is the high-position end (the top of a vertical axis).
fn end_overhang(
    labels: &[AxisLabel],
    half_extent: impl Fn(&AxisLabel) -> Pixels,
    extent: Pixels,
    flipped: bool,
) -> (Pixels, Pixels) {
    let extent = extent.max(Pixels::ZERO);
    let mut low = Pixels::ZERO;
    let mut high = Pixels::ZERO;
    for label in labels {
        let half = half_extent(label);
        let position = label.position.clamp(0.0, 1.0);
        low = low.max(half - extent * position + LABEL_SLACK);
        high = high.max(half - extent * (1.0 - position) + LABEL_SLACK);
    }
    if flipped { (high, low) } else { (low, high) }
}

#[cfg(test)]
mod tests {
    use gpui::{Bounds, Pixels, SharedString, point, px, size};

    use super::Axes;
    use crate::axes::horizontal::HorizontalAxis;
    use crate::axes::label::AxisLabel;
    use crate::axes::style::AxisStyle;
    use crate::axes::vertical::VerticalAxis;

    fn bounds() -> Bounds<Pixels> {
        Bounds::new(point(px(0.0), px(0.0)), size(px(200.0), px(100.0)))
    }

    fn no_text(_: &SharedString, _: &AxisStyle) -> Pixels {
        px(0.0)
    }

    #[test]
    fn no_axes_uses_full_bounds() {
        assert_eq!(Axes::default().frame(bounds(), no_text), bounds());
    }

    #[test]
    fn end_labels_pull_the_frame_in() {
        let style = AxisStyle::default().with_gutter(px(0.0));
        let labels = vec![AxisLabel::new(0.0, "start"), AxisLabel::new(1.0, "end")];
        let axes = Axes::new(Some(HorizontalAxis::new(labels).with_style(style)), None);
        // Every label measures 40px, so each end label overhangs by 20px.
        let frame = axes.frame(bounds(), |_, _| px(40.0));
        assert_eq!(frame.origin.x, px(22.0));
        assert_eq!(frame.right(), px(178.0));
    }

    #[test]
    fn vertical_end_labels_reserve_half_a_line() {
        let style = AxisStyle::default().with_gutter(px(0.0));
        let half_line = style.line_height() / 2.0;
        let labels = vec![AxisLabel::new(1.0, "top")];
        let axes = Axes::new(None, Some(VerticalAxis::new(labels).with_style(style)));
        let frame = axes.frame(bounds(), no_text);
        assert_eq!(frame.origin.y, half_line + px(2.0));
        assert_eq!(frame.bottom(), px(100.0));
    }

    #[test]
    fn axes_reserve_left_and_bottom_gutters() {
        let style = AxisStyle::default().with_gutter(px(30.0));
        let axes = Axes::new(
            Some(HorizontalAxis::new(Vec::new()).with_style(style.clone())),
            Some(VerticalAxis::new(Vec::new()).with_style(style)),
        );
        let frame = axes.frame(bounds(), no_text);
        assert_eq!(frame.origin, point(px(30.0), px(0.0)));
        assert_eq!(frame.size, size(px(170.0), px(70.0)));
    }
}
