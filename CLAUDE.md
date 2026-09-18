# gpui-charts

## Architecture

A chart component library for gpui:

- `src/lib.rs` — Library root; re-exports the public API and holds the crate docs.
- `src/graph.rs` — `Graph`, the gpui element (`RenderOnce`/`IntoElement`) that lays out axes and paints plots from a `canvas`.
- `src/plot.rs` + `src/plot/` — The object-safe `Plot` trait, normalized-to-pixel mapping helpers, and the concrete plots (points, line, bar).
- `src/axes.rs` + `src/axes/` — `Axes`, the horizontal/vertical axes, labels, style, and text painting.
- `src/builder.rs` + `src/builder/` — Raw builders that take normalized `0..=1` coordinates and validate them into `ChartError`.
- `src/series.rs` + `src/series/` — Friendly builders (time series, weekday, numeric x) that normalize real values and generate labels.
- `src/scrub.rs` — Scrubber options, results, the observable `ScrubState` entity, nearest-sample lookup, aggregation, and the default highlight painter. `Graph` registers the mouse listeners.
- `src/text.rs` — Shared single-line text measurement and painting (`LabelStyle`, `TextAnchor`).
- `src/point.rs`, `src/error.rs` — `NormalizedPoint` and `ChartError`.
- `src/bin/demo/main.rs` + `src/bin/demo/examples.rs` — A demo app that opens a window for visually testing components during development. No library logic lives here — it just exercises the library.

## After Each Change
Run the following commands after every code change and fix any issues before considering the change complete:

1. `cargo fmt` - Format all code
2. `cargo test` - Run all tests
3. `cargo clippy` - Run linter; fix all warnings and errors before completing the change

### Fixing Clippy Complexity Warnings
When clippy reports `cognitive_complexity`, `too_many_lines`, or `too_many_arguments` warnings, fix them by refactoring — never suppress with `#[allow]`:
- Extract logical sub-steps into well-named helper functions.
- When a file accumulates many functions, reorganize into helper files and structs (following the module conventions below).

## Dependencies
Always use exact versions for dependencies in `Cargo.toml` (e.g., `"4.5.60"` not `"4"`). Check `Cargo.lock` for the resolved version when pinning.

## Module Conventions
Never use `mod.rs`. Always use the modern Rust style: create a top-level file (e.g., `foo.rs`) as the module root, and a matching folder (`foo/`) for any submodules.

## Imports
Always use `use` imports rather than full crate paths at call sites. For example, prefer `use crate::chart::line;` + `line::LineChart` over `crate::chart::line::LineChart`.
