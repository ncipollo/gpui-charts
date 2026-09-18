//! Builder for [`Graph`].

use crate::axes::Axes;
use crate::graph::Graph;
use crate::plot::Plot;

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
    use gpui::{App, Bounds, Pixels, Window};

    use super::GraphBuilder;
    use crate::plot::Plot;

    struct NoopPlot;
    impl Plot for NoopPlot {
        fn paint(&self, _area: Bounds<Pixels>, _window: &mut Window, _cx: &mut App) {}
    }

    #[test]
    fn collects_plots_in_order() {
        let graph = GraphBuilder::new().plot(NoopPlot).plot(NoopPlot).build();
        assert_eq!(graph.plots().len(), 2);
    }
}
