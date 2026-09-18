//! Pointer-driven inspection of plot samples.
//!
//! Scrubbing is configured per plot with [`ScrubOptions`]. The graph asks
//! every enabled plot for the sample nearest the pointer, collects the answers
//! into [`ScrubResult`]s, paints each plot's highlight, and publishes the
//! results through a [`ScrubState`] entity so other views can display them.

pub mod search;

use gpui::{
    App, BorderStyle, Bounds, Context, Corners, Edges, Hsla, Pixels, SharedString, Window, fill,
    hsla, point, px, quad, size,
};

use crate::plot::{Plot, map_point, map_x};
use crate::point::NormalizedPoint;
use crate::scrub::search::SampleIndex;
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

/// How a scrubbed sample is drawn.
///
/// Colors left as `None` fall back to sensible defaults: the plot's own color
/// for the ring, translucent white for the guide, near-white for the value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrubStyle {
    /// Whether to draw a vertical guide line through the sample.
    pub show_guide: bool,
    /// Color of the guide line.
    pub guide_color: Option<Hsla>,
    /// Whether to draw a ring around the sample's point.
    pub show_point: bool,
    /// Color of the ring.
    pub point_color: Option<Hsla>,
    /// Color of the value text.
    pub value_color: Option<Hsla>,
}

impl Default for ScrubStyle {
    fn default() -> Self {
        Self {
            show_guide: true,
            guide_color: None,
            show_point: true,
            point_color: None,
            value_color: None,
        }
    }
}

/// Per-plot scrubber configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrubOptions {
    /// Whether this plot participates in scrubbing at all.
    pub enabled: bool,
    /// What causes the highlight to appear.
    pub trigger: ScrubTrigger,
    /// Whether the sample's value is drawn above the highlighted point.
    pub show_value: bool,
    /// How the highlight is drawn.
    pub style: ScrubStyle,
}

impl Default for ScrubOptions {
    /// Scrubbing is off by default.
    fn default() -> Self {
        Self {
            enabled: false,
            trigger: ScrubTrigger::Hover,
            show_value: true,
            style: ScrubStyle::default(),
        }
    }
}

impl ScrubOptions {
    /// Enabled, shown on hover, with the value drawn above the point.
    pub fn hover() -> Self {
        Self {
            enabled: true,
            trigger: ScrubTrigger::Hover,
            ..Self::default()
        }
    }

    /// Enabled, shown while the mouse is held down, with the value drawn.
    pub fn press_and_hold() -> Self {
        Self {
            enabled: true,
            trigger: ScrubTrigger::PressAndHold,
            ..Self::default()
        }
    }

    /// Sets whether the value is drawn above the highlighted point.
    pub fn with_show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Sets whether the vertical guide line is drawn.
    pub fn with_guide(mut self, show: bool) -> Self {
        self.style.show_guide = show;
        self
    }

    /// Sets the guide line color.
    pub fn with_guide_color(mut self, color: impl Into<Hsla>) -> Self {
        self.style.guide_color = Some(color.into());
        self
    }

    /// Sets whether the ring around the point is drawn.
    pub fn with_point(mut self, show: bool) -> Self {
        self.style.show_point = show;
        self
    }

    /// Sets the ring color (otherwise the plot's color is used).
    pub fn with_point_color(mut self, color: impl Into<Hsla>) -> Self {
        self.style.point_color = Some(color.into());
        self
    }

    /// Sets the value text color.
    pub fn with_value_color(mut self, color: impl Into<Hsla>) -> Self {
        self.style.value_color = Some(color.into());
        self
    }

    /// Replaces the whole highlight style.
    pub fn with_style(mut self, style: ScrubStyle) -> Self {
        self.style = style;
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
/// `samples` is the plot's [`SampleIndex`], which is searched rather than
/// scanned. `labels` supplies the text for each index; indices without a label
/// fall back to the formatted normalized `y`.
pub fn nearest(samples: &SampleIndex, labels: &[SharedString], x: f32) -> Option<ScrubSample> {
    let (index, point) = samples.nearest(x.clamp(0.0, 1.0))?;
    let label = labels
        .get(index)
        .cloned()
        .unwrap_or_else(|| SharedString::from(format_value(f64::from(point.y))));
    Some(ScrubSample {
        index,
        point,
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

/// Paints the standard highlight for a scrubbed sample as configured by
/// `options`: a guide line, a ring around the point, and the label above it.
/// `plot_color` is used for the ring when the style sets no color.
pub fn paint_highlight(
    area: Bounds<Pixels>,
    sample: &ScrubSample,
    plot_color: Hsla,
    options: &ScrubOptions,
    window: &mut Window,
    cx: &mut App,
) {
    let style = options.style;
    if style.show_guide {
        let x = map_x(area, sample.point.x);
        let guide = Bounds::new(point(x, area.top()), size(px(1.0), area.size.height));
        let color = style.guide_color.unwrap_or(hsla(0.0, 0.0, 1.0, 0.25));
        window.paint_quad(fill(guide, color));
    }

    let center = map_point(area, sample.point);
    let ring_radius = px(7.0);
    if style.show_point {
        let ring = quad(
            Bounds::centered_at(center, size(ring_radius * 2.0, ring_radius * 2.0)),
            Corners::all(ring_radius),
            hsla(0.0, 0.0, 0.0, 0.0),
            Edges::all(px(2.0)),
            style.point_color.unwrap_or(plot_color),
            BorderStyle::Solid,
        );
        window.paint_quad(ring);
    }

    if options.show_value {
        let anchor = point(center.x, center.y - ring_radius - px(2.0));
        let label_style = LabelStyle {
            color: style.value_color.unwrap_or(hsla(0.0, 0.0, 0.95, 1.0)),
            ..LabelStyle::default()
        };
        paint_text(
            &sample.label,
            anchor,
            TextAnchor::BottomCenter,
            &label_style,
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use gpui::{App, Bounds, Pixels, SharedString, Window};

    use super::search::SampleIndex;
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

    fn samples() -> SampleIndex {
        SampleIndex::new(&points())
    }

    #[test]
    fn nearest_picks_closest_x_and_formats_y() {
        let sample = nearest(&samples(), &[], 0.6).expect("non-empty");
        assert_eq!(sample.index, 1);
        assert_eq!(sample.label.as_ref(), "0.5");
    }

    #[test]
    fn nearest_uses_labels_and_clamps() {
        let labels = [SharedString::from("a"), SharedString::from("b")];
        let sample = nearest(&samples(), &labels, 5.0).expect("non-empty");
        assert_eq!(sample.index, 2);
        assert_eq!(sample.label.as_ref(), "0.9");
        let sample = nearest(&samples(), &labels, -1.0).expect("non-empty");
        assert_eq!(sample.label.as_ref(), "a");
    }

    #[test]
    fn style_setters_override_defaults() {
        let options = ScrubOptions::hover()
            .with_guide(false)
            .with_point_color(gpui::white())
            .with_value_color(gpui::black());
        assert!(!options.style.show_guide);
        assert!(options.style.show_point);
        assert_eq!(options.style.point_color, Some(gpui::white()));
        assert_eq!(options.style.value_color, Some(gpui::black()));
        assert_eq!(ScrubOptions::default().style.point_color, None);
    }

    #[test]
    fn nearest_of_nothing_is_none() {
        assert!(nearest(&SampleIndex::default(), &[], 0.5).is_none());
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
            nearest(&samples(), &[], x)
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
                ..ScrubOptions::default()
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
