//! Choosing which plot types a series renders as.

use gpui::Hsla;

use crate::builder::plot::{BarPlotBuilder, LinePlotBuilder, PointsPlotBuilder};
use crate::error::ChartError;
use crate::plot::Plot;
use crate::point::NormalizedPoint;

/// The kind of plot a series is rendered with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlotKind {
    /// A marker at each sample.
    Points,
    /// A line connecting the samples.
    Line,
    /// A bar from the baseline to each sample.
    Bar,
}

/// Shared rendering options for a series.
#[derive(Clone, Debug, Default)]
pub(crate) struct SeriesStyle {
    pub kinds: Vec<PlotKind>,
    pub color: Option<Hsla>,
    pub bar_width: Option<f32>,
}

impl SeriesStyle {
    /// Builds one plot per requested kind (falling back to `default_kind`).
    pub fn build_plots(
        &self,
        points: &[NormalizedPoint],
        default_kind: PlotKind,
    ) -> Result<Vec<Box<dyn Plot>>, ChartError> {
        let kinds: &[PlotKind] = if self.kinds.is_empty() {
            &[default_kind]
        } else {
            &self.kinds
        };
        kinds
            .iter()
            .map(|kind| self.build_plot(*kind, points))
            .collect()
    }

    fn build_plot(
        &self,
        kind: PlotKind,
        points: &[NormalizedPoint],
    ) -> Result<Box<dyn Plot>, ChartError> {
        let points = points.iter().copied();
        Ok(match kind {
            PlotKind::Points => {
                let mut b = PointsPlotBuilder::new().points(points);
                if let Some(color) = self.color {
                    b = b.color(color);
                }
                Box::new(b.build()?)
            }
            PlotKind::Line => {
                let mut b = LinePlotBuilder::new().points(points);
                if let Some(color) = self.color {
                    b = b.color(color);
                }
                Box::new(b.build()?)
            }
            PlotKind::Bar => {
                let mut b = BarPlotBuilder::new().bars(points);
                if let Some(color) = self.color {
                    b = b.color(color);
                }
                if let Some(width) = self.bar_width {
                    b = b.width(width);
                }
                Box::new(b.build()?)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PlotKind, SeriesStyle};
    use crate::point::NormalizedPoint;

    #[test]
    fn falls_back_to_default_kind() {
        let plots = SeriesStyle::default()
            .build_plots(&[NormalizedPoint::new(0.5, 0.5)], PlotKind::Bar)
            .expect("valid");
        assert_eq!(plots.len(), 1);
    }

    #[test]
    fn builds_one_plot_per_kind() {
        let style = SeriesStyle {
            kinds: vec![PlotKind::Line, PlotKind::Points],
            ..SeriesStyle::default()
        };
        let plots = style
            .build_plots(&[NormalizedPoint::new(0.5, 0.5)], PlotKind::Bar)
            .expect("valid");
        assert_eq!(plots.len(), 2);
    }
}
