//! The [`Graph`] component: plots plus axes, renderable in any gpui layout.

use std::rc::Rc;

use gpui::{
    App, Bounds, Element, ElementId, Entity, GlobalElementId, Hitbox, HitboxBehavior,
    InspectorElementId, IntoElement, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, RenderOnce, Size, Style, Window, px,
};

use crate::axes::{Axes, measure_label};
use crate::plot::Plot;
use crate::scrub::{ScrubState, aggregate};

/// Default inset between the axis baselines and the plot marks.
pub const DEFAULT_PLOT_PADDING: Pixels = px(6.0);

/// A chart composed of an ordered list of plots and a set of axes.
///
/// `Graph` implements [`IntoElement`], so it can be dropped into any gpui
/// layout like a `div`. It fills the size its parent gives it.
///
/// # Scrubbing
///
/// Plots opt into scrubbing individually through their
/// [`ScrubOptions`](crate::scrub::ScrubOptions). The graph needs somewhere
/// to keep the results: either pass an entity with
/// [`Graph::with_scrub_state`] (and observe it to show values elsewhere), or
/// give the graph an id with [`Graph::with_id`] and it keeps its own state.
/// Without either, scrub options have no effect.
#[derive(IntoElement)]
pub struct Graph {
    plots: Rc<Vec<Box<dyn Plot>>>,
    axes: Axes,
    plot_padding: Pixels,
    id: Option<ElementId>,
    scrub_state: Option<Entity<ScrubState>>,
}

impl Graph {
    /// Creates a graph from its plots and axes with the default plot padding.
    pub fn new(plots: Vec<Box<dyn Plot>>, axes: Axes) -> Self {
        Self {
            plots: Rc::new(plots),
            axes,
            plot_padding: DEFAULT_PLOT_PADDING,
            id: None,
            scrub_state: None,
        }
    }

    /// Sets the inset between the axis baselines and the plot marks, so marks
    /// at the extremes of the data do not sit on top of the axes.
    pub fn with_plot_padding(mut self, padding: Pixels) -> Self {
        self.plot_padding = padding;
        self
    }

    /// Gives the graph a stable id so it can keep its own scrub state.
    pub fn with_id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Publishes scrub results to `state`, which callers can observe.
    pub fn with_scrub_state(mut self, state: Entity<ScrubState>) -> Self {
        self.scrub_state = Some(state);
        self
    }

    /// The plots drawn by this graph, in draw order.
    pub fn plots(&self) -> &[Box<dyn Plot>] {
        &self.plots
    }

    /// The axes drawn around the plot area.
    pub fn axes(&self) -> &Axes {
        &self.axes
    }

    /// The inset between the axis baselines and the plot marks.
    pub fn plot_padding(&self) -> Pixels {
        self.plot_padding
    }

    /// Whether any plot has scrubbing enabled.
    pub fn is_scrubbable(&self) -> bool {
        self.plots.iter().any(|plot| plot.scrub_options().enabled)
    }
}

impl RenderOnce for Graph {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let scrubbable = self.is_scrubbable();
        let scrub_state = match (self.scrub_state, self.id, scrubbable) {
            (Some(state), _, _) => Some(state),
            (None, Some(id), true) => {
                Some(window.use_keyed_state(id, cx, |_, _| ScrubState::default()))
            }
            _ => None,
        };
        GraphElement {
            plots: self.plots,
            axes: self.axes,
            plot_padding: self.plot_padding,
            scrub_state,
        }
    }
}

/// The element behind [`Graph`], with its scrub state resolved.
struct GraphElement {
    plots: Rc<Vec<Box<dyn Plot>>>,
    axes: Axes,
    plot_padding: Pixels,
    scrub_state: Option<Entity<ScrubState>>,
}

/// Geometry computed during prepaint and reused while painting.
struct GraphLayout {
    frame: Bounds<Pixels>,
    area: Bounds<Pixels>,
    hitbox: Hitbox,
}

impl IntoElement for GraphElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for GraphElement {
    type RequestLayoutState = ();
    type PrepaintState = GraphLayout;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            size: Size::full(),
            ..Style::default()
        };
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let frame = self
            .axes
            .frame(bounds, |text, style| measure_label(text, style, window));
        let area = frame.inset(self.plot_padding);
        let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);
        GraphLayout {
            frame,
            area,
            hitbox,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.axes.paint(layout.frame, layout.area, window, cx);
        for plot in self.plots.iter() {
            plot.paint(layout.area, window, cx);
        }
        let Some(state) = self.scrub_state.clone() else {
            return;
        };
        for result in state.read(cx).results().to_vec() {
            if let Some(plot) = self.plots.get(result.plot) {
                plot.paint_scrub(layout.area, &result.sample, window, cx);
            }
        }
        let scrubber = Scrubber {
            plots: self.plots.clone(),
            area: layout.area,
            hitbox: layout.hitbox.clone(),
            state,
        };
        scrubber.listen(window);
    }
}

/// Mouse handling for one painted graph.
#[derive(Clone)]
struct Scrubber {
    plots: Rc<Vec<Box<dyn Plot>>>,
    area: Bounds<Pixels>,
    hitbox: Hitbox,
    state: Entity<ScrubState>,
}

impl Scrubber {
    fn listen(self, window: &mut Window) {
        let moved = self.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase.bubble() {
                moved.update(event.position, None, window, cx);
            }
        });
        let pressed = self.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase.bubble() && event.button == MouseButton::Left {
                pressed.update(event.position, Some(true), window, cx);
            }
        });
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase.bubble() && event.button == MouseButton::Left {
                self.update(event.position, Some(false), window, cx);
            }
        });
    }

    /// Recomputes the results for the pointer at `position`. `pressed`
    /// overrides the stored button state when a press or release happened.
    fn update(
        &self,
        position: Point<Pixels>,
        pressed: Option<bool>,
        window: &Window,
        cx: &mut App,
    ) {
        let hovered = self.hitbox.is_hovered(window);
        let was_pressed = self.state.read(cx).is_pressed();
        let pressed = match pressed {
            Some(true) => hovered,
            Some(false) => false,
            None => was_pressed,
        };
        let results = if hovered || pressed {
            aggregate(&self.plots, self.normalized_x(position), pressed)
        } else {
            Vec::new()
        };
        self.state
            .update(cx, |state, cx| state.set(results, pressed, cx));
    }

    fn normalized_x(&self, position: Point<Pixels>) -> f32 {
        let width = f32::from(self.area.size.width);
        if width <= 0.0 {
            return 0.0;
        }
        (f32::from(position.x - self.area.left()) / width).clamp(0.0, 1.0)
    }
}
