//! The horizontal (bottom) axis.

use gpui::{App, Bounds, Pixels, Window, fill, point, px, size};

use crate::axes::label::AxisLabel;
use crate::axes::style::{AxisStyle, TextAnchor, paint_label};
use crate::plot::map_x;

/// An axis drawn along the bottom edge of the plot area.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalAxis {
    labels: Vec<AxisLabel>,
    style: AxisStyle,
}

impl HorizontalAxis {
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

    /// Paints the baseline, ticks, and labels below `plot_area`.
    pub fn paint(&self, plot_area: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        let baseline_y = plot_area.bottom();
        let baseline = Bounds::new(
            point(plot_area.left(), baseline_y),
            size(plot_area.size.width, px(1.0)),
        );
        window.paint_quad(fill(baseline, self.style.line_color));

        let tick_length = self.style.tick_length;
        for label in &self.labels {
            let x = map_x(plot_area, label.position);
            let tick = Bounds::new(point(x, baseline_y), size(px(1.0), tick_length));
            window.paint_quad(fill(tick, self.style.line_color));
            let anchor = point(x, baseline_y + tick_length + px(2.0));
            paint_label(
                &label.text,
                anchor,
                TextAnchor::TopCenter,
                &self.style,
                window,
                cx,
            );
        }
    }
}
