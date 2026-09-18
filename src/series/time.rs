//! A series of timestamped values over a period.

use std::rc::Rc;

use gpui::Hsla;

use crate::builder::axes::AxesBuilder;
use crate::builder::graph::GraphBuilder;
use crate::error::ChartError;
use crate::graph::Graph;
use crate::point::NormalizedPoint;
use crate::series::axis::ValueAxis;
use crate::series::plots::{PlotKind, SeriesStyle};

/// Builds a graph from `(unix_seconds, value)` samples.
///
/// The period defaults to the span between the earliest and latest sample and
/// can be fixed with [`TimeSeriesBuilder::period`]. Timestamps are plain
/// `i64` Unix seconds so no date library is required; labels default to
/// `YYYY-MM-DD` in UTC and can be customised with a formatter.
pub struct TimeSeriesBuilder {
    samples: Vec<(i64, f64)>,
    period: Option<(i64, i64)>,
    x_label_count: usize,
    x_formatter: Rc<dyn Fn(i64) -> String>,
    y_axis: ValueAxis,
    style: SeriesStyle,
}

impl Default for TimeSeriesBuilder {
    fn default() -> Self {
        Self {
            samples: Vec::new(),
            period: None,
            x_label_count: 5,
            x_formatter: Rc::new(format_date),
            y_axis: ValueAxis::default(),
            style: SeriesStyle::default(),
        }
    }
}

impl TimeSeriesBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one sample at `timestamp` (Unix seconds).
    pub fn sample(mut self, timestamp: i64, value: f64) -> Self {
        self.samples.push((timestamp, value));
        self
    }

    /// Appends many samples.
    pub fn samples(mut self, samples: impl IntoIterator<Item = (i64, f64)>) -> Self {
        self.samples.extend(samples);
        self
    }

    /// Fixes the period shown on the `x` axis (Unix seconds).
    pub fn period(mut self, start: i64, end: i64) -> Self {
        self.period = Some((start, end));
        self
    }

    /// Sets how many evenly spaced time labels to generate (default 5).
    pub fn x_label_count(mut self, count: usize) -> Self {
        self.x_label_count = count;
        self
    }

    /// Sets the formatter used for time labels.
    pub fn x_formatter(mut self, format: impl Fn(i64) -> String + 'static) -> Self {
        self.x_formatter = Rc::new(format);
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

    /// Normalizes the samples over the period and builds the graph.
    pub fn build(mut self) -> Result<Graph, ChartError> {
        self.samples.sort_by_key(|s| s.0);
        let format = self.x_formatter.clone();
        let mut x_axis = ValueAxis::default()
            .label_count(self.x_label_count)
            .formatter(move |v| format(v.round() as i64));
        if let Some((start, end)) = self.period {
            x_axis = x_axis.range(start as f64, end as f64);
        }
        let (x_norm, x_labels) = x_axis.resolve(self.samples.iter().map(|s| s.0 as f64));
        let (y_norm, y_labels) = self.y_axis.resolve(self.samples.iter().map(|s| s.1));
        let points: Vec<NormalizedPoint> = self
            .samples
            .iter()
            .map(|(t, v)| NormalizedPoint::new(x_norm.normalize(*t as f64), y_norm.normalize(*v)))
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

/// Formats Unix seconds as a UTC `YYYY-MM-DD` date.
pub fn format_date(unix_seconds: i64) -> String {
    let (year, month, day) = civil_from_days(unix_seconds.div_euclid(86_400));
    format!("{year:04}-{month:02}-{day:02}")
}

/// Converts days since 1970-01-01 to a proleptic Gregorian `(year, month, day)`.
///
/// This is Howard Hinnant's `civil_from_days` algorithm.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::{TimeSeriesBuilder, format_date};

    #[test]
    fn formats_known_dates() {
        assert_eq!(format_date(0), "1970-01-01");
        assert_eq!(format_date(951_782_400), "2000-02-29");
        assert_eq!(format_date(1_700_000_000), "2023-11-14");
        assert_eq!(format_date(-86_400), "1969-12-31");
    }

    #[test]
    fn first_and_last_samples_span_the_axis() {
        let graph = TimeSeriesBuilder::new()
            .samples([(200, 2.0), (100, 1.0), (300, 3.0)])
            .x_label_count(2)
            .build()
            .expect("valid");
        let labels = graph.axes().horizontal().expect("x axis").labels();
        assert_eq!(labels[0].position, 0.0);
        assert_eq!(labels[1].position, 1.0);
    }

    #[test]
    fn fixed_period_and_custom_formatter() {
        let graph = TimeSeriesBuilder::new()
            .sample(50, 1.0)
            .period(0, 100)
            .x_label_count(3)
            .x_formatter(|t| format!("t{t}"))
            .build()
            .expect("valid");
        let labels = graph.axes().horizontal().expect("x axis").labels();
        let texts: Vec<&str> = labels.iter().map(|l| l.text.as_ref()).collect();
        assert_eq!(texts, ["t0", "t50", "t100"]);
    }
}
