//! Shared single-line text measurement and painting.

use gpui::{App, Hsla, Pixels, Point, ShapedLine, SharedString, TextRun, Window, hsla, point, px};

/// Visual settings for a painted label.
#[derive(Clone, Debug, PartialEq)]
pub struct LabelStyle {
    /// Text color.
    pub color: Hsla,
    /// Font size.
    pub font_size: Pixels,
}

impl Default for LabelStyle {
    fn default() -> Self {
        Self {
            color: hsla(0.0, 0.0, 0.75, 1.0),
            font_size: px(11.0),
        }
    }
}

impl LabelStyle {
    /// The line height used when painting.
    pub fn line_height(&self) -> Pixels {
        self.font_size * 1.3
    }
}

/// Where a label sits relative to its anchor point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAnchor {
    /// The anchor is the top-center of the text (used below a horizontal axis).
    TopCenter,
    /// The anchor is the bottom-center of the text (used above a scrubbed point).
    BottomCenter,
    /// The anchor is the middle-right of the text (used left of a vertical axis).
    MiddleRight,
}

/// Returns the painted width of `text` in `style`.
pub fn measure_text(text: &SharedString, style: &LabelStyle, window: &Window) -> Pixels {
    shape(text, style, window).width
}

fn shape(text: &SharedString, style: &LabelStyle, window: &Window) -> ShapedLine {
    let run = TextRun {
        len: text.len(),
        font: window.text_style().font(),
        color: style.color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(text.clone(), style.font_size, &[run], None)
}

/// Paints a single line of text anchored at `anchor`.
pub fn paint_text(
    text: &SharedString,
    anchor: Point<Pixels>,
    placement: TextAnchor,
    style: &LabelStyle,
    window: &mut Window,
    cx: &mut App,
) {
    let line = shape(text, style, window);
    let line_height = style.line_height();
    let origin = match placement {
        TextAnchor::TopCenter => point(anchor.x - line.width / 2.0, anchor.y),
        TextAnchor::BottomCenter => point(anchor.x - line.width / 2.0, anchor.y - line_height),
        TextAnchor::MiddleRight => point(anchor.x - line.width, anchor.y - line_height / 2.0),
    };
    // Painting only fails for malformed layouts; a missing label is not fatal.
    let _ = line.paint(origin, line_height, window, cx);
}
