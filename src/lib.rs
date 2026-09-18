//! Chart components for [gpui](https://www.gpui.rs/).
//!
//! The crate is organised around three abstractions:
//!
//! - [`Plot`]: one type per kind of mark to draw ([`PointsPlot`], [`LinePlot`],
//!   [`BarPlot`]). Overlays and highlights are plots too.
//! - [`Axes`]: the optional [`HorizontalAxis`] and [`VerticalAxis`] with their
//!   [`AxisLabel`]s.
//! - [`Graph`]: a gpui element composed of an ordered list of plots and a set
//!   of axes. It fills whatever size its parent layout gives it.
//!
//! # Coordinates
//!
//! All low-level data is expressed in [`NormalizedPoint`]s, where `x` and `y`
//! each lie in `0.0..=1.0`; `(0, 0)` is the bottom-left of the plot area and
//! `(1, 1)` the top-right. The raw builders ([`GraphBuilder`], [`AxesBuilder`],
//! [`PointsPlotBuilder`], [`LinePlotBuilder`], [`BarPlotBuilder`]) take those
//! coordinates directly and validate them, reporting a [`ChartError`] on the
//! first out-of-range value.
//!
//! # Friendly builders
//!
//! The [`series`] module maps real-world values onto normalized space and
//! generates axis labels for you:
//!
//! - [`TimeSeriesBuilder`] for `(unix_seconds, value)` samples over a period.
//! - [`WeekdayBuilder`] for one value per [`Weekday`].
//! - [`NumericSeriesBuilder`] for `(x, y)` samples on a non-time `x` axis.
//!
//! # Example
//!
//! ```
//! use gpui_charts::{PlotKind, TimeSeriesBuilder, ValueAxis};
//!
//! let graph = TimeSeriesBuilder::new()
//!     .samples([(1_700_000_000, 3.0), (1_700_086_400, 5.5), (1_700_172_800, 4.0)])
//!     .y_axis(ValueAxis::default().range(0.0, 10.0))
//!     .plot_kind(PlotKind::Line)
//!     .plot_kind(PlotKind::Points)
//!     .build()
//!     .expect("samples are valid");
//!
//! // `graph` implements `IntoElement`; drop it into any gpui layout:
//! // div().size_full().child(graph)
//! assert_eq!(graph.plots().len(), 2);
//! ```
//!
//! The raw builders give full control when you already have normalized data:
//!
//! ```
//! use gpui_charts::{AxesBuilder, BarPlotBuilder, GraphBuilder};
//!
//! let bars = BarPlotBuilder::new()
//!     .bars([(0.25, 0.4), (0.5, 0.9), (0.75, 0.6)])
//!     .build()?;
//! let axes = AxesBuilder::new()
//!     .horizontal_labels([(0.25, "a"), (0.5, "b"), (0.75, "c")])
//!     .vertical_labels([(0.0, "0"), (1.0, "100")])
//!     .build()?;
//! let graph = GraphBuilder::new().plot(bars).axes(axes).build();
//! assert_eq!(graph.plots().len(), 1);
//! # Ok::<(), gpui_charts::ChartError>(())
//! ```

pub mod axes;
pub mod builder;
pub mod error;
pub mod graph;
pub mod plot;
pub mod point;
pub mod series;

pub use axes::horizontal::HorizontalAxis;
pub use axes::label::evenly_spaced_labels;
pub use axes::vertical::VerticalAxis;
pub use axes::{Axes, AxisLabel, AxisStyle};
pub use builder::{AxesBuilder, BarPlotBuilder, GraphBuilder, LinePlotBuilder, PointsPlotBuilder};
pub use error::ChartError;
pub use graph::Graph;
pub use plot::bar::BarPlot;
pub use plot::line::LinePlot;
pub use plot::points::PointsPlot;
pub use plot::{Plot, map_point, map_x, map_y};
pub use point::NormalizedPoint;
pub use series::axis::format_value;
pub use series::time::format_date;
pub use series::{
    Normalizer, NumericSeriesBuilder, PlotKind, TimeSeriesBuilder, ValueAxis, Weekday,
    WeekdayBuilder,
};
