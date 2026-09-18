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
/// values passed to [`ValueAxis::resolve`]. An inferred range is widened to
/// "nice" bounds (multiples of 1, 2, or 5 times a power of ten) so the labels
/// read as round numbers; see [`ValueAxis::nice_ticks`].
#[derive(Clone)]
pub struct ValueAxis {
    range: Option<(f64, f64)>,
    label_count: usize,
    nice_ticks: bool,
    formatter: LabelFormatter,
}

impl fmt::Debug for ValueAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueAxis")
            .field("range", &self.range)
            .field("label_count", &self.label_count)
            .field("nice_ticks", &self.nice_ticks)
            .finish_non_exhaustive()
    }
}

impl Default for ValueAxis {
    fn default() -> Self {
        Self {
            range: None,
            label_count: 5,
            nice_ticks: true,
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

    /// Sets how many labels to aim for (default 5).
    ///
    /// With nice ticks the actual count may differ slightly, since the step is
    /// rounded to a round number.
    pub fn label_count(mut self, count: usize) -> Self {
        self.label_count = count;
        self
    }

    /// Chooses whether an inferred range is widened to round-number ticks
    /// (default `true`). When `false`, labels are spread evenly across the
    /// exact observed range. Fixed ranges always use their exact bounds.
    pub fn nice_ticks(mut self, nice: bool) -> Self {
        self.nice_ticks = nice;
        self
    }

    /// Sets the label formatter.
    pub fn formatter(mut self, format: impl Fn(f64) -> String + 'static) -> Self {
        self.formatter = Rc::new(format);
        self
    }

    /// Formats a value with this axis's formatter.
    pub fn format(&self, value: f64) -> String {
        (self.formatter)(value)
    }

    /// Resolves the normalizer and labels for this axis given observed values.
    ///
    /// When no range was fixed and there are no values, the range `0..=1` is
    /// used so an empty series still produces a sensible axis.
    pub fn resolve(&self, values: impl IntoIterator<Item = f64>) -> (Normalizer, Vec<AxisLabel>) {
        let observed = match self.range {
            Some((min, max)) => Normalizer::new(min, max),
            None => Normalizer::from_values(values).unwrap_or_else(|| Normalizer::new(0.0, 1.0)),
        };
        if self.label_count == 0 {
            return (observed, Vec::new());
        }
        let format = &self.formatter;
        if self.range.is_none() && self.nice_ticks {
            let ticks = nice_ticks(observed.min(), observed.max(), self.label_count);
            let normalizer = Normalizer::new(ticks.min, ticks.max);
            let labels = ticks
                .values()
                .map(|v| AxisLabel::new(normalizer.normalize(v), format(v)))
                .collect();
            return (normalizer, labels);
        }
        let labels = evenly_spaced_labels(observed.min(), observed.max(), self.label_count, |v| {
            format(v)
        });
        (observed, labels)
    }
}

/// A round-number tick sequence covering a range.
#[derive(Clone, Copy, Debug, PartialEq)]
struct NiceTicks {
    min: f64,
    max: f64,
    step: f64,
}

impl NiceTicks {
    /// The tick values from `min` to `max` inclusive.
    fn values(self) -> impl Iterator<Item = f64> {
        let count = ((self.max - self.min) / self.step).round() as usize;
        (0..=count).map(move |i| self.min + self.step * i as f64)
    }
}

/// Widens `min..=max` to round-number bounds with roughly `target` ticks.
///
/// The step is 1, 2, or 5 times a power of ten (Heckbert's "nice numbers").
fn nice_ticks(min: f64, max: f64, target: usize) -> NiceTicks {
    let intervals = target.saturating_sub(1).max(1) as f64;
    let mut step = nice_step((max - min) / intervals);
    let mut nice_min = (min / step).floor() * step;
    let mut nice_max = (max / step).ceil() * step;
    if nice_min == nice_max {
        // A single value: give it a step of room on both sides.
        step = nice_step(step);
        nice_min -= step;
        nice_max += step;
    }
    NiceTicks {
        min: nice_min,
        max: nice_max,
        step,
    }
}

/// Rounds `rough` up to the nearest 1, 2, or 5 times a power of ten.
fn nice_step(rough: f64) -> f64 {
    if !rough.is_finite() || rough <= 0.0 {
        return 1.0;
    }
    let magnitude = 10f64.powf(rough.log10().floor());
    let fraction = rough / magnitude;
    let nice = if fraction < 1.5 {
        1.0
    } else if fraction < 3.0 {
        2.0
    } else if fraction < 7.0 {
        5.0
    } else {
        10.0
    };
    nice * magnitude
}

/// Default number formatting: up to six decimals with trailing zeros trimmed,
/// so `3.0` reads `3`, `2.5` reads `2.5`, and `0.1 + 0.2` reads `0.3`.
pub fn format_value(value: f64) -> String {
    let text = format!("{value:.6}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{ValueAxis, format_value, nice_step, nice_ticks};
    use crate::axes::AxisLabel;

    fn texts(labels: &[AxisLabel]) -> Vec<&str> {
        labels.iter().map(|l| l.text.as_ref()).collect()
    }

    #[test]
    fn inferred_range_uses_nice_ticks() {
        let (n, labels) = ValueAxis::default().resolve([0.0, 110.25]);
        assert_eq!((n.min(), n.max()), (0.0, 120.0));
        assert_eq!(texts(&labels), ["0", "20", "40", "60", "80", "100", "120"]);
        assert_eq!(labels[0].position, 0.0);
        assert_eq!(labels[6].position, 1.0);
    }

    #[test]
    fn nice_ticks_can_be_disabled() {
        let (n, labels) = ValueAxis::default()
            .nice_ticks(false)
            .label_count(3)
            .resolve([2.0, 8.0, 5.0]);
        assert_eq!((n.min(), n.max()), (2.0, 8.0));
        assert_eq!(texts(&labels), ["2", "5", "8"]);
    }

    #[test]
    fn nice_step_rounds_to_1_2_5() {
        assert_eq!(nice_step(0.35), 0.5);
        assert_eq!(nice_step(1.2), 1.0);
        assert_eq!(nice_step(2.0), 2.0);
        assert_eq!(nice_step(36.75), 50.0);
        assert_eq!(nice_step(80.0), 100.0);
        assert_eq!(nice_step(0.0), 1.0);
    }

    #[test]
    fn single_value_gets_room_on_both_sides() {
        let ticks = nice_ticks(5.0, 5.0, 5);
        assert!(ticks.min < 5.0 && ticks.max > 5.0);
        assert!(ticks.values().count() >= 2);
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
        assert_eq!(format_value(2.5), "2.5");
        assert_eq!(format_value(0.1 + 0.2), "0.3");
        assert_eq!(format_value(-0.0), "0");
    }
}
