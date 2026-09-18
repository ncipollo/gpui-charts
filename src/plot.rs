//! The [`Plot`] abstraction: one implementation per kind of mark.
//!
//! Concrete plots (points, line, bar) live in submodules of this module.
//! Highlights and overlays are expressed as plots too, so a [`Graph`] only ever
//! deals with a list of `dyn Plot`.
//!
//! [`Graph`]: crate::Graph

/// A drawable layer of a graph.
///
/// The render entry point is added in a follow-up; for now the trait only
/// establishes the object-safe shape that a graph holds.
pub trait Plot {}
