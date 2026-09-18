//! User-friendly builders that accept real-world values.
//!
//! These sit on top of the raw builders in [`crate::builder`]. They take
//! timestamps, weekday buckets, or arbitrary numeric `x` values, work out the
//! normalization onto `0.0..=1.0`, and generate axis labels.
//!
//! - [`TimeSeriesBuilder`]: `(timestamp, value)` samples over a period.
//! - [`WeekdayBuilder`]: one value per day of the week.
//! - [`NumericSeriesBuilder`]: `(x, y)` samples on a non-time `x` axis.

pub mod axis;
pub mod normalize;
pub mod numeric;
pub mod plots;
pub mod time;
pub mod weekday;

pub use axis::ValueAxis;
pub use normalize::Normalizer;
pub use numeric::NumericSeriesBuilder;
pub use plots::PlotKind;
pub use time::TimeSeriesBuilder;
pub use weekday::{Weekday, WeekdayBuilder};
