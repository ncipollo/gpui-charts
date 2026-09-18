//! The horizontal (bottom) axis.

use gpui::{App, Bounds, Pixels, Window, fill, point, px, size};

use crate::axes::label::AxisLabel;
use crate::axes::style::AxisStyle;
use crate::plot::map_x;
use crate::text::{TextAnchor, paint_text};

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

    /// Paints the baseline along the bottom of `frame`, with ticks and labels
    /// lined up to normalized positions inside `area`.
    pub fn paint(
        &self,
        frame: Bounds<Pixels>,
        area: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let baseline_y = frame.bottom();
        let baseline = Bounds::new(
            point(frame.left(), baseline_y),
            size(frame.size.width, px(1.0)),
        );
        window.paint_quad(fill(baseline, self.style.line_color));

        let tick_length = self.style.tick_length;
        for label in &self.labels {
            let x = map_x(area, label.position);
            let tick = Bounds::new(point(x, baseline_y), size(px(1.0), tick_length));
            window.paint_quad(fill(tick, self.style.line_color));
            let anchor = point(x, baseline_y + tick_length + px(2.0));
            paint_text(
                &label.text,
                anchor,
                TextAnchor::TopCenter,
                &self.style.label_style(),
                window,
                cx,
            );
        }
    }
}
