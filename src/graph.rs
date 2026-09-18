//! The [`Graph`] component: plots plus axes, renderable in any gpui layout.

use gpui::{App, Bounds, IntoElement, Pixels, RenderOnce, Window, canvas, prelude::*, px};

use crate::axes::{Axes, measure_label};
use crate::plot::Plot;

/// Default inset between the axis baselines and the plot marks.
pub const DEFAULT_PLOT_PADDING: Pixels = px(6.0);

/// A chart composed of an ordered list of plots and a set of axes.
///
/// `Graph` implements [`IntoElement`], so it can be dropped into any gpui
/// layout like a `div`. It fills the size its parent gives it.
#[derive(IntoElement)]
pub struct Graph {
    plots: Vec<Box<dyn Plot>>,
    axes: Axes,
    plot_padding: Pixels,
}

impl Graph {
    /// Creates a graph from its plots and axes with the default plot padding.
    pub fn new(plots: Vec<Box<dyn Plot>>, axes: Axes) -> Self {
        Self {
            plots,
            axes,
            plot_padding: DEFAULT_PLOT_PADDING,
        }
    }

    /// Sets the inset between the axis baselines and the plot marks, so marks
    /// at the extremes of the data do not sit on top of the axes.
    pub fn with_plot_padding(mut self, padding: Pixels) -> Self {
        self.plot_padding = padding;
        self
    }

    /// The inset between the axis baselines and the plot marks.
    pub fn plot_padding(&self) -> Pixels {
        self.plot_padding
    }

    /// The plots drawn by this graph, in draw order.
    pub fn plots(&self) -> &[Box<dyn Plot>] {
        &self.plots
    }

    /// The axes drawn around the plot area.
    pub fn axes(&self) -> &Axes {
        &self.axes
    }

    /// Paints the axes and then every plot, in order, within `bounds`.
    ///
    /// The axes reserve their gutters and label overhang first, giving the
    /// frame the baselines are drawn on. Plots (and the axis ticks) use the
    /// frame inset by the plot padding.
    fn paint(&self, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        let frame = self
            .axes
            .frame(bounds, |text, style| measure_label(text, style, window));
        let area = frame.inset(self.plot_padding);
        self.axes.paint(frame, area, window, cx);
        for plot in &self.plots {
            plot.paint(area, window, cx);
        }
    }
}

impl RenderOnce for Graph {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        canvas(
            |_bounds, _window, _cx| (),
            move |bounds, (), window, cx| self.paint(bounds, window, cx),
        )
        .size_full()
    }
}
