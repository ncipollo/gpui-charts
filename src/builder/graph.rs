//! Builder for [`Graph`].

use gpui::Pixels;

use crate::axes::Axes;
use crate::graph::Graph;
use crate::plot::Plot;

/// Builds a [`Graph`] from plots and axes.
#[derive(Default)]
pub struct GraphBuilder {
    plots: Vec<Box<dyn Plot>>,
    axes: Axes,
    plot_padding: Option<Pixels>,
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

    /// Appends an already boxed plot.
    pub fn plot_boxed(mut self, plot: Box<dyn Plot>) -> Self {
        self.plots.push(plot);
        self
    }

    /// Sets the axes drawn around the plot area.
    pub fn axes(mut self, axes: Axes) -> Self {
        self.axes = axes;
        self
    }

    /// Sets the inset between the axis baselines and the plot marks.
    pub fn plot_padding(mut self, padding: Pixels) -> Self {
        self.plot_padding = Some(padding);
        self
    }

    /// Finishes building the graph.
    pub fn build(self) -> Graph {
        let graph = Graph::new(self.plots, self.axes);
        match self.plot_padding {
            Some(padding) => graph.with_plot_padding(padding),
            None => graph,
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::{App, Bounds, Pixels, Window, px};

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

    #[test]
    fn applies_plot_padding() {
        let graph = GraphBuilder::new().plot_padding(px(0.0)).build();
        assert_eq!(graph.plot_padding(), px(0.0));
    }
}
