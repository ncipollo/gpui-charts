//! Mapping a numeric range onto `0.0..=1.0`.

/// Maps values in `min..=max` onto normalized `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normalizer {
    min: f64,
    max: f64,
}

impl Normalizer {
    /// Creates a normalizer for `min..=max`. The bounds are swapped if reversed.
    pub fn new(min: f64, max: f64) -> Self {
        if min <= max {
            Self { min, max }
        } else {
            Self { min: max, max: min }
        }
    }

    /// Creates a normalizer spanning the observed values, or `None` if empty.
    pub fn from_values(values: impl IntoIterator<Item = f64>) -> Option<Self> {
        let mut iter = values.into_iter();
        let first = iter.next()?;
        let (min, max) = iter.fold((first, first), |(lo, hi), v| (lo.min(v), hi.max(v)));
        Some(Self::new(min, max))
    }

    /// The lower bound.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// The upper bound.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Normalizes `value`; a degenerate range (min == max) maps to the center.
    pub fn normalize(&self, value: f64) -> f32 {
        let span = self.max - self.min;
        if span == 0.0 {
            return 0.5;
        }
        (((value - self.min) / span).clamp(0.0, 1.0)) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::Normalizer;

    #[test]
    fn maps_bounds_to_zero_and_one() {
        let n = Normalizer::new(10.0, 20.0);
        assert_eq!(n.normalize(10.0), 0.0);
        assert_eq!(n.normalize(20.0), 1.0);
        assert_eq!(n.normalize(15.0), 0.5);
    }

    #[test]
    fn degenerate_range_centers() {
        assert_eq!(Normalizer::new(3.0, 3.0).normalize(3.0), 0.5);
    }

    #[test]
    fn from_values_spans_min_to_max() {
        let n = Normalizer::from_values([4.0, -1.0, 9.0]).expect("non-empty");
        assert_eq!((n.min(), n.max()), (-1.0, 9.0));
        assert!(Normalizer::from_values(Vec::<f64>::new()).is_none());
    }

    #[test]
    fn clamps_outside_values() {
        let n = Normalizer::new(0.0, 1.0);
        assert_eq!(n.normalize(-5.0), 0.0);
        assert_eq!(n.normalize(5.0), 1.0);
    }
}
