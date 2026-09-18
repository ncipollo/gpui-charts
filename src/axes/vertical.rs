//! The vertical (left) axis.

use gpui::{App, Bounds, Pixels, Window, fill, point, px, size};

use crate::axes::label::AxisLabel;
use crate::axes::style::{AxisStyle, TextAnchor, paint_label};
use crate::plot::map_y;

/// An axis drawn along the left edge of the plot area.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VerticalAxis {
    labels: Vec<AxisLabel>,
    style: AxisStyle,
}

impl VerticalAxis {
    /// Creates an axis with the given labels and default styling.
    pub fn new(labels: Vec<AxisLabel>) -> Self {
        Self {
            labels,
            style: AxisStyle::default(),
        }
    }

    /// Sets the axis style.
    pub fn with_style(mut self, style: AxisStyle) -> Self {
        self.style = style;
        self
    }

    /// The labels along this axis.
    pub fn labels(&self) -> &[AxisLabel] {
        &self.labels
    }

    /// The axis style.
    pub fn style(&self) -> &AxisStyle {
        &self.style
    }

    /// Paints the baseline, ticks, and labels left of `plot_area`.
    pub fn paint(&self, plot_area: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        let baseline_x = plot_area.left() - px(1.0);
        let baseline = Bounds::new(
            point(baseline_x, plot_area.top()),
            size(px(1.0), plot_area.size.height),
        );
        window.paint_quad(fill(baseline, self.style.line_color));

        let tick_length = self.style.tick_length;
        for label in &self.labels {
            let y = map_y(plot_area, label.position);
            let tick = Bounds::new(
                point(baseline_x - tick_length, y),
                size(tick_length, px(1.0)),
            );
            window.paint_quad(fill(tick, self.style.line_color));
            let anchor = point(baseline_x - tick_length - px(4.0), y);
            paint_label(
                &label.text,
                anchor,
                TextAnchor::MiddleRight,
                &self.style,
                window,
                cx,
            );
        }
    }
}
