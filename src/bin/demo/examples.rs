//! Sample graphs, one per plot type and builder.
//!
//! All data is hard-coded so the demo is deterministic.

use gpui::{Hsla, hsla, px};
use gpui_charts::{
    AxesBuilder, BarPlotBuilder, Graph, GraphBuilder, LinePlotBuilder, NumericSeriesBuilder,
    PlotKind, PointsPlotBuilder, TimeSeriesBuilder, ValueAxis, Weekday, WeekdayBuilder,
};

const DAY: i64 = 86_400;
const START: i64 = 1_700_000_000;

fn blue() -> Hsla {
    hsla(0.58, 0.75, 0.6, 1.0)
}

fn orange() -> Hsla {
    hsla(0.08, 0.85, 0.6, 1.0)
}

fn green() -> Hsla {
    hsla(0.38, 0.6, 0.5, 1.0)
}

/// Every example, paired with its title, in display order.
pub fn all() -> Vec<(&'static str, Graph)> {
    vec![
        ("Scatter (PointsPlot)", scatter()),
        ("Line (LinePlot)", line()),
        ("Line + points overlay", line_with_points()),
        ("Bars (BarPlot)", bars()),
        ("Time series over a period", time_series()),
        ("Day of week", weekday()),
        ("Numeric x axis", numeric()),
    ]
}

fn scatter_points() -> [(f32, f32); 10] {
    [
        (0.05, 0.2),
        (0.15, 0.55),
        (0.25, 0.35),
        (0.35, 0.8),
        (0.45, 0.5),
        (0.55, 0.65),
        (0.65, 0.3),
        (0.75, 0.9),
        (0.85, 0.45),
        (0.95, 0.7),
    ]
}

fn wave_points() -> Vec<(f32, f32)> {
    (0..=20)
        .map(|i| {
            let x = i as f32 / 20.0;
            let y = 0.5 + 0.4 * (x * std::f32::consts::TAU).sin();
            (x, y)
        })
        .collect()
}

fn percent_axes() -> gpui_charts::Axes {
    AxesBuilder::new()
        .horizontal_labels([(0.0, "0"), (0.5, "0.5"), (1.0, "1")])
        .vertical_labels([(0.0, "0%"), (0.5, "50%"), (1.0, "100%")])
        .build()
        .expect("static labels are in range")
}

fn scatter() -> Graph {
    let points = PointsPlotBuilder::new()
        .points(scatter_points())
        .color(blue())
        .radius(px(5.0))
        .build()
        .expect("static points are in range");
    GraphBuilder::new()
        .plot(points)
        .axes(percent_axes())
        .build()
}

fn line() -> Graph {
    let line = LinePlotBuilder::new()
        .points(wave_points())
        .color(orange())
        .build()
        .expect("static points are in range");
    GraphBuilder::new().plot(line).axes(percent_axes()).build()
}

fn line_with_points() -> Graph {
    let line = LinePlotBuilder::new()
        .points(wave_points())
        .color(orange())
        .build()
        .expect("static points are in range");
    let points = PointsPlotBuilder::new()
        .points(wave_points())
        .color(blue())
        .radius(px(3.0))
        .build()
        .expect("static points are in range");
    GraphBuilder::new()
        .plot(line)
        .plot(points)
        .axes(percent_axes())
        .build()
}

fn bars() -> Graph {
    let bars = BarPlotBuilder::new()
        .bars([
            (0.1, 0.4),
            (0.3, 0.75),
            (0.5, 0.55),
            (0.7, 0.95),
            (0.9, 0.3),
        ])
        .color(green())
        .build()
        .expect("static bars are in range");
    let axes = AxesBuilder::new()
        .horizontal_labels([(0.1, "A"), (0.3, "B"), (0.5, "C"), (0.7, "D"), (0.9, "E")])
        .vertical_labels([(0.0, "0"), (0.5, "50"), (1.0, "100")])
        .build()
        .expect("static labels are in range");
    GraphBuilder::new().plot(bars).axes(axes).build()
}

fn time_series() -> Graph {
    let values = [12.0, 15.5, 14.0, 18.2, 21.0, 19.5, 23.1, 22.0, 25.4, 24.0];
    TimeSeriesBuilder::new()
        .samples(
            values
                .iter()
                .enumerate()
                .map(|(i, v)| (START + i as i64 * DAY, *v)),
        )
        .x_label_count(4)
        .y_axis(ValueAxis::default().range(0.0, 30.0).label_count(4))
        .plot_kind(PlotKind::Line)
        .plot_kind(PlotKind::Points)
        .color(blue())
        .build()
        .expect("static samples are valid")
}

fn weekday() -> Graph {
    WeekdayBuilder::new()
        .values([
            (Weekday::Monday, 42.0),
            (Weekday::Tuesday, 58.0),
            (Weekday::Wednesday, 51.0),
            (Weekday::Thursday, 73.0),
            (Weekday::Friday, 66.0),
            (Weekday::Saturday, 30.0),
            (Weekday::Sunday, 21.0),
        ])
        .y_axis(ValueAxis::default().range(0.0, 80.0))
        .color(green())
        .build()
        .expect("static values are valid")
}

fn numeric() -> Graph {
    NumericSeriesBuilder::new()
        .samples((0..=12).map(|i| {
            let x = f64::from(i) * 2.5;
            (x, (x * 0.35).powi(2))
        }))
        .x_axis(ValueAxis::default().label_count(4))
        .y_axis(ValueAxis::default().label_count(4))
        .plot_kind(PlotKind::Line)
        .color(orange())
        .build()
        .expect("static samples are valid")
}
