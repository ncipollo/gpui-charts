//! Shared styling helpers for plots.

use gpui::{Hsla, hsla};

/// The default color used by plots that do not set one.
pub fn default_color() -> Hsla {
    hsla(0.6, 0.7, 0.55, 1.0)
}
