//! Horizontal and vertical axes with their labels.

/// The axes rendered around a graph's plot area.
///
/// Axis data structures and label series are added in a follow-up; this
/// placeholder lets a [`Graph`] own an `Axes` value today.
///
/// [`Graph`]: crate::Graph
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Axes {}
