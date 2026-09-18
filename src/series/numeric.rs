//! A series with an arbitrary numeric (non-time) `x` axis.

use gpui::Hsla;

use crate::builder::axes::AxesBuilder;
use crate::builder::graph::GraphBuilder;
use crate::error::ChartError;
use crate::graph::Graph;
use crate::point::NormalizedPoint;
use crate::series::axis::ValueAxis;
use crate::series::plots::{PlotKind, SeriesStyle};

/// Builds a graph from `(x, y)` samples where `x` is any numeric quantity.
///
/// Both axes are normalized from the observed (or fixed) ranges and labelled
/// with evenly spaced values. Samples are sorted by `x` before plotting.
#[derive(Debug, Default)]
pub struct NumericSeriesBuilder {
    samples: Vec<(f64, f64)>,
    x_axis: ValueAxis,
    y_axis: ValueAxis,
    style: SeriesStyle,
}

impl NumericSeriesBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one sample.
    pub fn sample(mut self, x: f64, y: f64) -> Self {
        self.samples.push((x, y));
        self
    }

    /// Appends many samples.
    pub fn samples(mut self, samples: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.samples.extend(samples);
        self
    }

    /// Configures the `x` axis range, label count, and formatter.
    pub fn x_axis(mut self, axis: ValueAxis) -> Self {
        self.x_axis = axis;
        self
    }

    /// Configures the `y` axis range, label count, and formatter.
    pub fn y_axis(mut self, axis: ValueAxis) -> Self {
        self.y_axis = axis;
        self
    }

    /// Adds a plot kind to render; defaults to a line when none are set.
    pub fn plot_kind(mut self, kind: PlotKind) -> Self {
        self.style.kinds.push(kind);
        self
    }

    /// Sets the color used by every plot in the series.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.style.color = Some(color.into());
        self
    }

    /// Normalizes the samples and builds the graph.
    pub fn build(mut self) -> Result<Graph, ChartError> {
        self.samples.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (x_norm, x_labels) = self.x_axis.resolve(self.samples.iter().map(|s| s.0));
        let (y_norm, y_labels) = self.y_axis.resolve(self.samples.iter().map(|s| s.1));
        let points: Vec<NormalizedPoint> = self
            .samples
            .iter()
            .map(|(x, y)| NormalizedPoint::new(x_norm.normalize(*x), y_norm.normalize(*y)))
            .collect();
        let axes = AxesBuilder::new()
            .horizontal_labels(x_labels)
            .vertical_labels(y_labels)
            .build()?;
        let mut graph = GraphBuilder::new().axes(axes);
        for plot in self.style.build_plots(&points, PlotKind::Line)? {
            graph = graph.plot_boxed(plot);
        }
        Ok(graph.build())
    }
}

#[cfg(test)]
mod tests {
    use super::NumericSeriesBuilder;
    use crate::series::axis::ValueAxis;

    #[test]
    fn builds_axes_and_a_line() {
        let graph = NumericSeriesBuilder::new()
            .samples([(3.0, 30.0), (1.0, 10.0), (2.0, 20.0)])
            .x_axis(ValueAxis::default().label_count(3))
            .build()
            .expect("valid");
        assert_eq!(graph.plots().len(), 1);
        let x_labels = graph.axes().horizontal().expect("x axis").labels();
        let texts: Vec<&str> = x_labels.iter().map(|l| l.text.as_ref()).collect();
        assert_eq!(texts, ["1", "2", "3"]);
    }
}
