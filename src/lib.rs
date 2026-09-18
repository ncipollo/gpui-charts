//! Chart components for gpui.
//!
//! The crate is organised around three abstractions:
//!
//! - [`Plot`]: one type per kind of mark to draw (points, lines, bars).
//! - [`Axes`]: the horizontal and vertical axes and their labels.
//! - [`Graph`]: a gpui element composed of a set of plots and axes.
//!
//! All low-level data is expressed in [`NormalizedPoint`]s, where `x` and `y`
//! each lie in the `0.0..=1.0` range. Higher-level builders map real-world
//! values (timestamps, categories) onto that range.

pub mod axes;
pub mod builder;
pub mod graph;
pub mod plot;
pub mod point;

pub use axes::horizontal::HorizontalAxis;
pub use axes::label::evenly_spaced_labels;
pub use axes::vertical::VerticalAxis;
pub use axes::{Axes, AxisLabel, AxisStyle};
pub use builder::{AxesBuilder, GraphBuilder};
pub use graph::Graph;
pub use plot::bar::BarPlot;
pub use plot::line::LinePlot;
pub use plot::points::PointsPlot;
pub use plot::{Plot, map_point, map_x, map_y};
pub use point::NormalizedPoint;
