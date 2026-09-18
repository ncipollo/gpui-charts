# gpui-charts

Chart components for [gpui](https://www.gpui.rs/), Zed's GPU-accelerated UI framework.

## Requirements

- Stable Rust (edition 2024)
- macOS with Xcode installed and selected (`sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`) — gpui renders with Metal

## Usage

`Graph` is a gpui element: build one and drop it into any layout.

```rust
use gpui::{div, prelude::*};
use gpui_charts::{PlotKind, TimeSeriesBuilder, ValueAxis};

let graph = TimeSeriesBuilder::new()
    .samples([(1_700_000_000, 3.0), (1_700_086_400, 5.5), (1_700_172_800, 4.0)])
    .y_axis(ValueAxis::default().range(0.0, 10.0))
    .plot_kind(PlotKind::Line)
    .plot_kind(PlotKind::Points)
    .build()?;

div().size_full().child(graph)
```

Three friendly builders normalize real-world values and label the axes for
you: `TimeSeriesBuilder`, `WeekdayBuilder`, and `NumericSeriesBuilder`. When
you already have coordinates in the `0..=1` range, the raw `GraphBuilder`,
`AxesBuilder`, and per-plot builders (`PointsPlotBuilder`, `LinePlotBuilder`,
`BarPlotBuilder`) take them directly.

## Demo

    cargo run --bin demo

Opens a window for visually testing chart components as they're built.

## Develop

    cargo fmt
    cargo test
    cargo clippy
