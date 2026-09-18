//! A series with one value per day of the week.

use gpui::Hsla;

use crate::builder::axes::AxesBuilder;
use crate::builder::graph::GraphBuilder;
use crate::error::ChartError;
use crate::graph::Graph;
use crate::point::NormalizedPoint;
use crate::scrub::ScrubOptions;
use crate::series::axis::ValueAxis;
use crate::series::plots::{PlotKind, SeriesStyle};

/// A day of the week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Weekday {
    /// All days, Monday through Sunday.
    pub const ALL: [Weekday; 7] = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];

    /// A three-letter abbreviation, e.g. `"Mon"`.
    pub fn short_name(self) -> &'static str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }

    /// The day's slot (`0..7`) when the week starts on `start`.
    pub fn slot_from(self, start: Weekday) -> usize {
        (self.index() + 7 - start.index()) % 7
    }

    fn index(self) -> usize {
        Weekday::ALL
            .iter()
            .position(|d| *d == self)
            .expect("every weekday is in ALL")
    }
}

/// Builds a graph with seven evenly spaced weekday buckets on the `x` axis.
///
/// Each bucket is centered at `(slot + 0.5) / 7`. Days without a value are
/// left empty. Renders as bars unless another plot kind is chosen.
#[derive(Debug)]
pub struct WeekdayBuilder {
    values: Vec<(Weekday, f64)>,
    start: Weekday,
    y_axis: ValueAxis,
    style: SeriesStyle,
}

impl Default for WeekdayBuilder {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            start: Weekday::Monday,
            y_axis: ValueAxis::default(),
            style: SeriesStyle::default(),
        }
    }
}

impl WeekdayBuilder {
    /// Creates an empty builder starting the week on Monday.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the value for one day.
    pub fn value(mut self, day: Weekday, value: f64) -> Self {
        self.values.push((day, value));
        self
    }

    /// Sets values for many days.
    pub fn values(mut self, values: impl IntoIterator<Item = (Weekday, f64)>) -> Self {
        self.values.extend(values);
        self
    }

    /// Sets which day occupies the first slot.
    pub fn start_day(mut self, start: Weekday) -> Self {
        self.start = start;
        self
    }

    /// Configures the `y` axis range, label count, and formatter.
    pub fn y_axis(mut self, axis: ValueAxis) -> Self {
        self.y_axis = axis;
        self
    }

    /// Adds a plot kind to render; defaults to bars when none are set.
    pub fn plot_kind(mut self, kind: PlotKind) -> Self {
        self.style.kinds.push(kind);
        self
    }

    /// Sets the color used by every plot in the series.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.style.color = Some(color.into());
        self
    }

    /// Sets the scrubber configuration for every plot in the series. Scrubbed
    /// values are labelled with the `y` axis formatter.
    pub fn scrub(mut self, scrub: ScrubOptions) -> Self {
        self.style.scrub = scrub;
        self
    }

    /// Normalizes the values and builds the graph.
    pub fn build(mut self) -> Result<Graph, ChartError> {
        self.values
            .sort_by_key(|(day, _)| day.slot_from(self.start));
        let (y_norm, y_labels) = self.y_axis.resolve(self.values.iter().map(|v| v.1));
        let points: Vec<NormalizedPoint> = self
            .values
            .iter()
            .map(|(day, value)| {
                NormalizedPoint::new(
                    slot_center(day.slot_from(self.start)),
                    y_norm.normalize(*value),
                )
            })
            .collect();
        let x_labels = (0..7).map(|slot| {
            let day = Weekday::ALL[(self.start.index() + slot) % 7];
            (slot_center(slot), day.short_name())
        });
        let axes = AxesBuilder::new()
            .horizontal_labels(x_labels)
            .vertical_labels(y_labels)
            .build()?;
        self.style.bar_width.get_or_insert(0.6 / 7.0);
        self.style.labels = self
            .values
            .iter()
            .map(|v| self.y_axis.format(v.1))
            .map(Into::into)
            .collect();
        let mut graph = GraphBuilder::new().axes(axes);
        for plot in self.style.build_plots(&points, PlotKind::Bar)? {
            graph = graph.plot_boxed(plot);
        }
        Ok(graph.build())
    }
}

/// The normalized `x` at the center of weekday slot `slot`.
fn slot_center(slot: usize) -> f32 {
    (slot as f32 + 0.5) / 7.0
}

#[cfg(test)]
mod tests {
    use super::{Weekday, WeekdayBuilder, slot_center};

    #[test]
    fn slots_follow_the_start_day() {
        assert_eq!(Weekday::Monday.slot_from(Weekday::Monday), 0);
        assert_eq!(Weekday::Sunday.slot_from(Weekday::Monday), 6);
        assert_eq!(Weekday::Sunday.slot_from(Weekday::Sunday), 0);
        assert_eq!(Weekday::Monday.slot_from(Weekday::Sunday), 1);
    }

    #[test]
    fn slot_centers_are_evenly_spaced() {
        assert!((slot_center(0) - 0.5 / 7.0).abs() < f32::EPSILON);
        assert!((slot_center(6) - 6.5 / 7.0).abs() < f32::EPSILON);
    }

    #[test]
    fn builds_seven_labels_from_start_day() {
        let graph = WeekdayBuilder::new()
            .start_day(Weekday::Sunday)
            .value(Weekday::Monday, 3.0)
            .build()
            .expect("valid");
        let labels = graph.axes().horizontal().expect("x axis").labels();
        assert_eq!(labels.len(), 7);
        assert_eq!(labels[0].text.as_ref(), "Sun");
        assert_eq!(labels[1].text.as_ref(), "Mon");
    }
}
