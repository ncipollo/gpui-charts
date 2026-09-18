//! Builders for assembling graphs, axes, and plots.
//!
//! The raw builders in this module accept normalized coordinates directly.
//! Friendlier builders that accept timestamps or categories are layered on top
//! of these in a follow-up.

use crate::axes::Axes;
use crate::graph::Graph;
use crate::plot::Plot;

/// Builds an [`Axes`] value from label series at normalized positions.
#[derive(Debug, Default)]
pub struct AxesBuilder {}

impl AxesBuilder {
    /// Creates an empty axes builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finishes building the axes.
    pub fn build(self) -> Axes {
        Axes::default()
    }
}

/// Builds a [`Graph`] from plots and axes.
#[derive(Default)]
pub struct GraphBuilder {
    plots: Vec<Box<dyn Plot>>,
    axes: Axes,
}

impl GraphBuilder {
    /// Creates an empty graph builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a plot; plots draw in the order they are added.
    pub fn plot(mut self, plot: impl Plot + 'static) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// Sets the axes drawn around the plot area.
    pub fn axes(mut self, axes: Axes) -> Self {
        self.axes = axes;
        self
    }

    /// Finishes building the graph.
    pub fn build(self) -> Graph {
        Graph::new(self.plots, self.axes)
    }
}

#[cfg(test)]
mod tests {
    use super::{AxesBuilder, GraphBuilder};
    use crate::plot::Plot;

    struct NoopPlot;
    impl Plot for NoopPlot {}

    #[test]
    fn graph_builder_collects_plots_in_order() {
        let graph = GraphBuilder::new()
            .plot(NoopPlot)
            .plot(NoopPlot)
            .axes(AxesBuilder::new().build())
            .build();
        assert_eq!(graph.plots().len(), 2);
    }
}
