//! The [`Graph`] component: plots plus axes, renderable in any gpui layout.

use gpui::{App, IntoElement, RenderOnce, Window, canvas, prelude::*};

use crate::axes::Axes;
use crate::plot::Plot;

/// A chart composed of an ordered list of plots and a set of axes.
///
/// `Graph` implements [`IntoElement`], so it can be dropped into any gpui
/// layout like a `div`. It fills the size its parent gives it.
#[derive(IntoElement)]
pub struct Graph {
    plots: Vec<Box<dyn Plot>>,
    axes: Axes,
}

impl Graph {
    /// Creates a graph from its plots and axes.
    pub fn new(plots: Vec<Box<dyn Plot>>, axes: Axes) -> Self {
        Self { plots, axes }
    }

    /// The plots drawn by this graph, in draw order.
    pub fn plots(&self) -> &[Box<dyn Plot>] {
        &self.plots
    }

    /// The axes drawn around the plot area.
    pub fn axes(&self) -> &Axes {
        &self.axes
    }
}

impl RenderOnce for Graph {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        canvas(|_bounds, _window, _cx| (), |_bounds, (), _window, _cx| {}).size_full()
    }
}
