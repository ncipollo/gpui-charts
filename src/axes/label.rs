//! Axis labels and helpers for generating them.

use gpui::SharedString;

/// A label placed at a normalized position along an axis.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisLabel {
    /// Position along the axis, `0.0` (left/bottom) to `1.0` (right/top).
    pub position: f32,
    /// The text to draw.
    pub text: SharedString,
}

impl AxisLabel {
    /// Creates a label at `position`.
    pub fn new(position: f32, text: impl Into<SharedString>) -> Self {
        Self {
            position,
            text: text.into(),
        }
    }
}

impl<T: Into<SharedString>> From<(f32, T)> for AxisLabel {
    fn from((position, text): (f32, T)) -> Self {
        Self::new(position, text)
    }
}

/// Generates `count` evenly spaced labels covering the value range `min..=max`.
///
/// The first label sits at position `0.0` with value `min` and the last at
/// `1.0` with value `max`. `format` turns each value into label text. Fewer
/// than two labels yields a single label at `min`.
pub fn evenly_spaced_labels(
    min: f64,
    max: f64,
    count: usize,
    format: impl Fn(f64) -> String,
) -> Vec<AxisLabel> {
    if count < 2 {
        return vec![AxisLabel::new(0.0, format(min))];
    }
    let steps = (count - 1) as f64;
    (0..count)
        .map(|i| {
            let fraction = i as f64 / steps;
            let value = min + (max - min) * fraction;
            AxisLabel::new(fraction as f32, format(value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::evenly_spaced_labels;

    #[test]
    fn spans_min_to_max() {
        let labels = evenly_spaced_labels(0.0, 100.0, 5, |v| format!("{v:.0}"));
        let rendered: Vec<(f32, &str)> = labels
            .iter()
            .map(|l| (l.position, l.text.as_ref()))
            .collect();
        assert_eq!(
            rendered,
            vec![
                (0.0, "0"),
                (0.25, "25"),
                (0.5, "50"),
                (0.75, "75"),
                (1.0, "100")
            ]
        );
    }

    #[test]
    fn single_label_sits_at_min() {
        let labels = evenly_spaced_labels(3.0, 9.0, 1, |v| format!("{v}"));
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].position, 0.0);
        assert_eq!(labels[0].text.as_ref(), "3");
    }
}
