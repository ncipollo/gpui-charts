//! Pointer-driven inspection of plot samples.
//!
//! Scrubbing is configured per plot with [`ScrubOptions`]. The graph asks
//! every enabled plot for the sample nearest the pointer, collects the answers
//! into [`ScrubResult`]s, paints each plot's highlight, and publishes the
//! results through a [`ScrubState`] entity so other views can display them.

use gpui::{
    App, BorderStyle, Bounds, Context, Corners, Edges, Hsla, Pixels, SharedString, Window, fill,
    hsla, point, px, quad, size,
};

use crate::plot::{Plot, map_point, map_x};
use crate::point::NormalizedPoint;
use crate::series::axis::format_value;
use crate::text::{LabelStyle, TextAnchor, paint_text};

/// What causes a plot's scrubber to show.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrubTrigger {
    /// Show while the pointer is over the graph.
    #[default]
    Hover,
    /// Show only while the primary mouse button is held down over the graph.
    PressAndHold,
}

/// Per-plot scrubber configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrubOptions {
    /// Whether this plot participates in scrubbing at all.
    pub enabled: bool,
    /// What causes the highlight to appear.
    pub trigger: ScrubTrigger,
    /// Whether the sample's value is drawn above the highlighted point.
    pub show_value: bool,
}

impl Default for ScrubOptions {
    /// Scrubbing is off by default.
    fn default() -> Self {
        Self {
            enabled: false,
            trigger: ScrubTrigger::Hover,
            show_value: true,
        }
    }
}

impl ScrubOptions {
    /// Enabled, shown on hover, with the value drawn above the point.
    pub fn hover() -> Self {
        Self {
            enabled: true,
            trigger: ScrubTrigger::Hover,
            show_value: true,
        }
    }

    /// Enabled, shown while the mouse is held down, with the value drawn.
    pub fn press_and_hold() -> Self {
        Self {
            enabled: true,
            trigger: ScrubTrigger::PressAndHold,
            show_value: true,
        }
    }

    /// Sets whether the value is drawn above the highlighted point.
    pub fn with_show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Whether the scrubber should show given the current pointer state.
    pub fn is_triggered(&self, pressed: bool) -> bool {
        self.enabled
            && match self.trigger {
                ScrubTrigger::Hover => true,
                ScrubTrigger::PressAndHold => pressed,
            }
    }
}

/// The sample of one plot nearest the pointer.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubSample {
    /// Index of the sample within the plot's data.
    pub index: usize,
    /// The sample's normalized position.
    pub point: NormalizedPoint,
    /// Text describing the sample's value.
    pub label: SharedString,
}

/// A scrub sample tagged with the plot it came from.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubResult {
    /// Index of the plot within the graph, in draw order.
    pub plot: usize,
    /// The plot's nearest sample.
    pub sample: ScrubSample,
}

/// Observable scrub results for one graph.
///
/// Create one with `cx.new(|_| ScrubState::default())`, pass it to the graph
/// via `GraphBuilder::scrub_state`, and observe it with `cx.observe` to react
/// when the results change.
#[derive(Debug, Default)]
pub struct ScrubState {
    results: Vec<ScrubResult>,
    pressed: bool,
}

impl ScrubState {
    /// The current results, one per plot that is actively scrubbed.
    pub fn results(&self) -> &[ScrubResult] {
        &self.results
    }

    /// The first result, convenient for single-plot graphs.
    pub fn first(&self) -> Option<&ScrubResult> {
        self.results.first()
    }

    /// Whether any plot is currently scrubbed.
    pub fn is_active(&self) -> bool {
        !self.results.is_empty()
    }

    /// Whether the primary mouse button is held down over the graph.
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    /// Replaces the results and pressed flag, notifying observers on change.
    pub(crate) fn set(&mut self, results: Vec<ScrubResult>, pressed: bool, cx: &mut Context<Self>) {
        if self.results != results || self.pressed != pressed {
            self.results = results;
            self.pressed = pressed;
            cx.notify();
        }
    }
}

/// Finds the sample whose `x` is nearest to `x`.
///
/// `labels` supplies the text for each index; indices without a label fall
/// back to the formatted normalized `y`.
pub fn nearest(points: &[NormalizedPoint], labels: &[SharedString], x: f32) -> Option<ScrubSample> {
    let x = x.clamp(0.0, 1.0);
    let (index, point) = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| (a.x - x).abs().total_cmp(&(b.x - x).abs()))?;
    let label = labels
        .get(index)
        .cloned()
        .unwrap_or_else(|| SharedString::from(format_value(f64::from(point.y))));
    Some(ScrubSample {
        index,
        point: *point,
        label,
    })
}

/// Collects the scrub result of every plot whose trigger is satisfied.
pub fn aggregate(plots: &[Box<dyn Plot>], x: f32, pressed: bool) -> Vec<ScrubResult> {
    plots
        .iter()
        .enumerate()
        .filter(|(_, plot)| plot.scrub_options().is_triggered(pressed))
        .filter_map(|(index, plot)| {
            plot.scrub(x).map(|sample| ScrubResult {
                plot: index,
                sample,
            })
        })
        .collect()
}

/// Paints the standard highlight for a scrubbed sample: a guide line, a ring
/// around the point in `color`, and optionally the label above it.
pub fn paint_highlight(
    area: Bounds<Pixels>,
    sample: &ScrubSample,
    color: Hsla,
    show_value: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let x = map_x(area, sample.point.x);
    let guide = Bounds::new(point(x, area.top()), size(px(1.0), area.size.height));
    window.paint_quad(fill(guide, hsla(0.0, 0.0, 1.0, 0.25)));

    let center = map_point(area, sample.point);
    let ring_radius = px(7.0);
    let ring = quad(
        Bounds::centered_at(center, size(ring_radius * 2.0, ring_radius * 2.0)),
        Corners::all(ring_radius),
        hsla(0.0, 0.0, 0.0, 0.0),
        Edges::all(px(2.0)),
        color,
        BorderStyle::Solid,
    );
    window.paint_quad(ring);

    if show_value {
        let anchor = point(center.x, center.y - ring_radius - px(2.0));
        let style = LabelStyle {
            color: hsla(0.0, 0.0, 0.95, 1.0),
            ..LabelStyle::default()
        };
        paint_text(
            &sample.label,
            anchor,
            TextAnchor::BottomCenter,
            &style,
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use gpui::{App, Bounds, Pixels, SharedString, Window};

    use super::{ScrubOptions, ScrubSample, ScrubTrigger, aggregate, nearest};
    use crate::plot::Plot;
    use crate::point::NormalizedPoint;

    fn points() -> Vec<NormalizedPoint> {
        vec![
            NormalizedPoint::new(0.0, 0.1),
            NormalizedPoint::new(0.5, 0.5),
            NormalizedPoint::new(1.0, 0.9),
        ]
    }

    #[test]
    fn nearest_picks_closest_x_and_formats_y() {
        let sample = nearest(&points(), &[], 0.6).expect("non-empty");
        assert_eq!(sample.index, 1);
        assert_eq!(sample.label.as_ref(), "0.5");
    }

    #[test]
    fn nearest_uses_labels_and_clamps() {
        let labels = [SharedString::from("a"), SharedString::from("b")];
        let sample = nearest(&points(), &labels, 5.0).expect("non-empty");
        assert_eq!(sample.index, 2);
        assert_eq!(sample.label.as_ref(), "0.9");
        let sample = nearest(&points(), &labels, -1.0).expect("non-empty");
        assert_eq!(sample.label.as_ref(), "a");
    }

    #[test]
    fn nearest_of_nothing_is_none() {
        assert!(nearest(&[], &[], 0.5).is_none());
    }

    #[test]
    fn triggers_respect_pressed_state() {
        assert!(ScrubOptions::hover().is_triggered(false));
        assert!(!ScrubOptions::press_and_hold().is_triggered(false));
        assert!(ScrubOptions::press_and_hold().is_triggered(true));
        assert!(!ScrubOptions::default().is_triggered(true));
    }

    struct StubPlot(ScrubOptions);

    impl Plot for StubPlot {
        fn paint(&self, _area: Bounds<Pixels>, _window: &mut Window, _cx: &mut App) {}

        fn scrub_options(&self) -> ScrubOptions {
            self.0
        }

        fn scrub(&self, x: f32) -> Option<ScrubSample> {
            nearest(&points(), &[], x)
        }
    }

    #[test]
    fn aggregate_respects_each_plots_options() {
        let plots: Vec<Box<dyn Plot>> = vec![
            Box::new(StubPlot(ScrubOptions::hover())),
            Box::new(StubPlot(ScrubOptions::default())),
            Box::new(StubPlot(ScrubOptions {
                enabled: true,
                trigger: ScrubTrigger::PressAndHold,
                show_value: false,
            })),
        ];
        let hovered: Vec<usize> = aggregate(&plots, 0.4, false)
            .iter()
            .map(|r| r.plot)
            .collect();
        assert_eq!(hovered, [0]);
        let pressed: Vec<usize> = aggregate(&plots, 0.4, true)
            .iter()
            .map(|r| r.plot)
            .collect();
        assert_eq!(pressed, [0, 2]);
    }
}
