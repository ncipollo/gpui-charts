//! Axis styling and the shared text painting helper.

use gpui::{App, Hsla, Pixels, Point, SharedString, TextRun, Window, hsla, point, px};

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
        Self {
            line_color: hsla(0.0, 0.0, 0.6, 1.0),
            text_color: hsla(0.0, 0.0, 0.75, 1.0),
            font_size: px(11.0),
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
        self.font_size * 1.3
    }
}

/// Where a label sits relative to its anchor point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAnchor {
    /// The anchor is the top-center of the text (used below a horizontal axis).
    TopCenter,
    /// The anchor is the middle-right of the text (used left of a vertical axis).
    MiddleRight,
}

/// Paints a single line of text anchored at `anchor`.
pub fn paint_label(
    text: &SharedString,
    anchor: Point<Pixels>,
    placement: TextAnchor,
    style: &AxisStyle,
    window: &mut Window,
    cx: &mut App,
) {
    let run = TextRun {
        len: text.len(),
        font: window.text_style().font(),
        color: style.text_color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let line = window
        .text_system()
        .shape_line(text.clone(), style.font_size, &[run], None);
    let line_height = style.line_height();
    let origin = match placement {
        TextAnchor::TopCenter => point(anchor.x - line.width / 2.0, anchor.y),
        TextAnchor::MiddleRight => point(anchor.x - line.width, anchor.y - line_height / 2.0),
    };
    // Painting only fails for malformed layouts; a missing label is not fatal.
    let _ = line.paint(origin, line_height, window, cx);
}
