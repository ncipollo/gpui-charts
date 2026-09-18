//! Configuration for a numeric axis derived from observed values.

use std::fmt;
use std::rc::Rc;

use crate::axes::label::{AxisLabel, evenly_spaced_labels};
use crate::series::normalize::Normalizer;

/// Formats an axis value as label text.
pub type LabelFormatter = Rc<dyn Fn(f64) -> String>;

/// Describes how a numeric axis is normalized and labelled.
///
/// The range is either fixed with [`ValueAxis::range`] or inferred from the
/// values passed to [`ValueAxis::resolve`].
#[derive(Clone)]
pub struct ValueAxis {
    range: Option<(f64, f64)>,
    label_count: usize,
    formatter: LabelFormatter,
}

impl fmt::Debug for ValueAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueAxis")
            .field("range", &self.range)
            .field("label_count", &self.label_count)
            .finish_non_exhaustive()
    }
}

impl Default for ValueAxis {
    fn default() -> Self {
        Self {
            range: None,
            label_count: 5,
            formatter: Rc::new(format_value),
        }
    }
}

impl ValueAxis {
    /// Fixes the axis range instead of inferring it from the data.
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range = Some((min, max));
        self
    }

    /// Sets how many evenly spaced labels to generate (default 5).
    pub fn label_count(mut self, count: usize) -> Self {
        self.label_count = count;
        self
    }

    /// Sets the label formatter.
    pub fn formatter(mut self, format: impl Fn(f64) -> String + 'static) -> Self {
        self.formatter = Rc::new(format);
        self
    }

    /// Resolves the normalizer and labels for this axis given observed values.
    ///
    /// When no range was fixed and there are no values, the range `0..=1` is
    /// used so an empty series still produces a sensible axis.
    pub fn resolve(&self, values: impl IntoIterator<Item = f64>) -> (Normalizer, Vec<AxisLabel>) {
        let normalizer = match self.range {
            Some((min, max)) => Normalizer::new(min, max),
            None => Normalizer::from_values(values).unwrap_or_else(|| Normalizer::new(0.0, 1.0)),
        };
        let labels = if self.label_count == 0 {
            Vec::new()
        } else {
            let format = &self.formatter;
            evenly_spaced_labels(normalizer.min(), normalizer.max(), self.label_count, |v| {
                format(v)
            })
        };
        (normalizer, labels)
    }
}

/// Default number formatting: integers without decimals, otherwise two places.
pub fn format_value(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
    }
}

#[cfg(test)]
mod tests {
    use super::{ValueAxis, format_value};

    #[test]
    fn infers_range_from_values() {
        let (n, labels) = ValueAxis::default().label_count(3).resolve([2.0, 8.0, 5.0]);
        assert_eq!((n.min(), n.max()), (2.0, 8.0));
        let texts: Vec<&str> = labels.iter().map(|l| l.text.as_ref()).collect();
        assert_eq!(texts, ["2", "5", "8"]);
    }

    #[test]
    fn fixed_range_wins_over_values() {
        let (n, _) = ValueAxis::default().range(0.0, 100.0).resolve([40.0]);
        assert_eq!((n.min(), n.max()), (0.0, 100.0));
    }

    #[test]
    fn empty_values_fall_back_to_unit_range() {
        let (n, _) = ValueAxis::default().resolve(Vec::<f64>::new());
        assert_eq!((n.min(), n.max()), (0.0, 1.0));
    }

    #[test]
    fn formats_integers_and_fractions() {
        assert_eq!(format_value(3.0), "3");
        assert_eq!(format_value(2.5), "2.50");
    }
}
