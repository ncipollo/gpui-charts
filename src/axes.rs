//! Horizontal and vertical axes with their labels.
//!
//! Axes are purely positional: each label sits at a normalized position along
//! its axis. Higher-level builders decide what the labels say.

pub mod horizontal;
pub mod label;
pub mod style;
pub mod vertical;

use gpui::{App, Bounds, Pixels, Window, point, size};

use crate::axes::horizontal::HorizontalAxis;
use crate::axes::vertical::VerticalAxis;

pub use label::AxisLabel;
pub use style::AxisStyle;

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

    /// Computes the plot area left over after reserving axis gutters.
    ///
    /// The vertical axis reserves a gutter on the left; the horizontal axis
    /// reserves one along the bottom.
    pub fn plot_area(&self, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        let left = self
            .vertical
            .as_ref()
            .map_or(Pixels::ZERO, |axis| axis.style().gutter);
        let bottom = self
            .horizontal
            .as_ref()
            .map_or(Pixels::ZERO, |axis| axis.style().gutter);
        let width = (bounds.size.width - left).max(Pixels::ZERO);
        let height = (bounds.size.height - bottom).max(Pixels::ZERO);
        Bounds::new(
            point(bounds.origin.x + left, bounds.origin.y),
            size(width, height),
        )
    }

    /// Paints both axes around `plot_area`.
    pub fn paint(&self, plot_area: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        if let Some(axis) = &self.horizontal {
            axis.paint(plot_area, window, cx);
        }
        if let Some(axis) = &self.vertical {
            axis.paint(plot_area, window, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Bounds, Pixels, point, px, size};

    use super::Axes;
    use crate::axes::horizontal::HorizontalAxis;
    use crate::axes::style::AxisStyle;
    use crate::axes::vertical::VerticalAxis;

    fn bounds() -> Bounds<Pixels> {
        Bounds::new(point(px(0.0), px(0.0)), size(px(200.0), px(100.0)))
    }

    #[test]
    fn no_axes_uses_full_bounds() {
        assert_eq!(Axes::default().plot_area(bounds()), bounds());
    }

    #[test]
    fn axes_reserve_left_and_bottom_gutters() {
        let style = AxisStyle::default().with_gutter(px(30.0));
        let axes = Axes::new(
            Some(HorizontalAxis::new(Vec::new()).with_style(style.clone())),
            Some(VerticalAxis::new(Vec::new()).with_style(style)),
        );
        let area = axes.plot_area(bounds());
        assert_eq!(area.origin, point(px(30.0), px(0.0)));
        assert_eq!(area.size, size(px(170.0), px(70.0)));
    }
}
