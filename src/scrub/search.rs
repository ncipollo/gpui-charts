//! Sorted lookup of the sample nearest a scrub position.

use crate::point::NormalizedPoint;

/// A plot's samples sorted by `x` so scrubbing can binary search them.
///
/// A plot builds one when it is created and keeps it for its lifetime; a
/// plot's points never change after construction, so the index cannot go
/// stale. Sorting costs `O(n log n)` once and buys an `O(log n)` lookup on
/// every pointer move, instead of a scan of every sample each time.
///
/// Each entry keeps the position its point had in the plot's own data, so a
/// lookup reports an index callers can use against the values they supplied.
#[derive(Clone, Debug, Default)]
pub struct SampleIndex {
    /// Every point paired with its index in the plot's data, ordered by
    /// ascending `x`. Points sharing an `x` stay in data order.
    entries: Vec<(NormalizedPoint, usize)>,
}

impl SampleIndex {
    /// Sorts `points` by `x`, remembering each one's original index.
    pub fn new(points: &[NormalizedPoint]) -> Self {
        let mut entries: Vec<(NormalizedPoint, usize)> = points
            .iter()
            .enumerate()
            .map(|(index, point)| (*point, index))
            .collect();
        entries
            .sort_by(|(a, a_index), (b, b_index)| a.x.total_cmp(&b.x).then(a_index.cmp(b_index)));
        Self { entries }
    }

    /// The number of indexed samples.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether there is nothing to scrub.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The sample whose `x` is nearest `x`, as its index in the plot's data
    /// and its position, or `None` when there are no samples.
    ///
    /// Equal distances resolve to the sample with the smaller `x`, and samples
    /// sharing that `x` to the one earliest in the plot's data.
    pub fn nearest(&self, x: f32) -> Option<(usize, NormalizedPoint)> {
        let split = self.entries.partition_point(|(point, _)| point.x < x);
        let before = split.checked_sub(1).map(|i| self.entries[i]);
        let after = self.entries.get(split).copied();
        let nearest = match (before, after) {
            (Some(before), Some(after)) if after.0.x - x < x - before.0.x => after,
            (Some(before), _) => before,
            (None, after) => after?,
        };
        Some(self.first_sharing_x(nearest))
    }

    /// The earliest sample in data order sharing `nearest`'s `x`, so that
    /// duplicated positions resolve the way a front-to-back scan would.
    fn first_sharing_x(&self, nearest: (NormalizedPoint, usize)) -> (usize, NormalizedPoint) {
        let (point, index) = nearest;
        let first = self.entries.partition_point(|(other, _)| other.x < point.x);
        match self.entries.get(first) {
            Some((first, first_index)) if first.x == point.x => (*first_index, *first),
            _ => (index, point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SampleIndex;
    use crate::point::NormalizedPoint;

    fn index(xs: &[f32]) -> SampleIndex {
        let points: Vec<NormalizedPoint> =
            xs.iter().map(|x| NormalizedPoint::new(*x, 0.5)).collect();
        SampleIndex::new(&points)
    }

    fn nearest_index(index: &SampleIndex, x: f32) -> Option<usize> {
        index.nearest(x).map(|(index, _)| index)
    }

    #[test]
    fn empty_index_finds_nothing() {
        let index = index(&[]);
        assert!(index.is_empty());
        assert_eq!(index.nearest(0.5), None);
    }

    #[test]
    fn finds_the_nearest_of_sorted_samples() {
        let index = index(&[0.0, 0.25, 0.5, 0.75, 1.0]);
        assert_eq!(index.len(), 5);
        assert_eq!(nearest_index(&index, 0.0), Some(0));
        assert_eq!(nearest_index(&index, 0.3), Some(1));
        assert_eq!(nearest_index(&index, 0.4), Some(2));
        assert_eq!(nearest_index(&index, 1.0), Some(4));
    }

    #[test]
    fn reports_indices_into_the_original_data() {
        let index = index(&[0.9, 0.1, 0.5]);
        assert_eq!(
            index.nearest(0.95),
            Some((0, NormalizedPoint::new(0.9, 0.5)))
        );
        assert_eq!(nearest_index(&index, 0.0), Some(1));
        assert_eq!(nearest_index(&index, 0.45), Some(2));
    }

    #[test]
    fn positions_outside_the_samples_clamp_to_the_ends() {
        let index = index(&[0.2, 0.4, 0.6]);
        assert_eq!(nearest_index(&index, -1.0), Some(0));
        assert_eq!(nearest_index(&index, 2.0), Some(2));
    }

    #[test]
    fn equal_distances_go_to_the_smaller_x() {
        // Halves and quarters are exact in `f32`, so this really is a tie.
        let index = index(&[0.25, 0.75]);
        assert_eq!(nearest_index(&index, 0.5), Some(0));
    }

    #[test]
    fn duplicate_x_resolves_to_the_first_in_data_order() {
        let index = index(&[0.5, 0.5, 0.5]);
        assert_eq!(nearest_index(&index, 0.5), Some(0));
        assert_eq!(nearest_index(&index, 0.9), Some(0));
        assert_eq!(nearest_index(&index, 0.1), Some(0));
    }

    #[test]
    fn single_sample_is_always_nearest() {
        let index = index(&[0.7]);
        assert_eq!(nearest_index(&index, 0.0), Some(0));
        assert_eq!(nearest_index(&index, 1.0), Some(0));
    }

    #[test]
    fn matches_a_linear_scan_over_unsorted_samples() {
        let xs = [0.62, 0.05, 0.99, 0.31, 0.5, 0.0, 0.77, 0.18];
        let index = index(&xs);
        for step in 0..=100 {
            let x = step as f32 / 100.0;
            let found = nearest_index(&index, x).expect("non-empty");
            let scanned = xs
                .iter()
                .map(|sample| (sample - x).abs())
                .min_by(f32::total_cmp)
                .expect("non-empty");
            assert_eq!((xs[found] - x).abs(), scanned, "at x = {x}");
        }
    }
}
