//! Axis styling.

use gpui::{Hsla, Pixels, SharedString, Window, hsla, px};

use crate::text::{LabelStyle, measure_text};

/// Visual settings shared by both axes.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisStyle {
    /// Color of the axis baseline and tick marks.
    pub line_color: Hsla,
    /// Color of label text.
    pub text_color: Hsla,
    /// Font size of label text.
    pub font_size: Pixels,
    /// Length of tick marks extending away from the plot area.
    pub tick_length: Pixels,
    /// Space reserved outside the plot area for this axis.
    pub gutter: Pixels,
}

impl Default for AxisStyle {
    fn default() -> Self {
        let text = LabelStyle::default();
        Self {
            line_color: hsla(0.0, 0.0, 0.6, 1.0),
            text_color: text.color,
            font_size: text.font_size,
            tick_length: px(4.0),
            gutter: px(36.0),
        }
    }
}

impl AxisStyle {
    /// Sets the baseline and tick color.
    pub fn with_line_color(mut self, color: impl Into<Hsla>) -> Self {
        self.line_color = color.into();
        self
    }

    /// Sets the label text color.
    pub fn with_text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_color = color.into();
        self
    }

    /// Sets the label font size.
    pub fn with_font_size(mut self, size: Pixels) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the gutter reserved for this axis.
    pub fn with_gutter(mut self, gutter: Pixels) -> Self {
        self.gutter = gutter;
        self
    }

    /// The line height used when painting labels.
    pub fn line_height(&self) -> Pixels {
        self.label_style().line_height()
    }

    /// The text style used for this axis's labels.
    pub fn label_style(&self) -> LabelStyle {
        LabelStyle {
            color: self.text_color,
            font_size: self.font_size,
        }
    }
}

/// Returns the painted width of an axis label in `style`.
pub fn measure_label(text: &SharedString, style: &AxisStyle, window: &Window) -> Pixels {
    measure_text(text, &style.label_style(), window)
}
