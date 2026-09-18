//! A demo app for visually testing gpui-charts components.
//!
//! Every plot type and builder in the library is exercised here. No library
//! logic lives in this binary; it only calls the public API.

mod examples;

use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, IntoElement, Render, SharedString,
    Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_charts::ScrubState;

use crate::examples::Example;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 900.0;
const CARD_WIDTH: f32 = 400.0;
const CARD_HEIGHT: f32 = 270.0;

const BACKGROUND: u32 = 0x1c1c1e;
const CARD_BACKGROUND: u32 = 0x2a2a2d;
const TITLE_COLOR: u32 = 0xe6e6e6;
const NOTE_COLOR: u32 = 0x8e8e93;
const VALUE_COLOR: u32 = 0x64d2ff;

struct DemoRoot {
    /// Scrub results for the time series card, echoed in its header.
    scrub: Entity<ScrubState>,
}

impl DemoRoot {
    fn new(cx: &mut Context<Self>) -> Self {
        let scrub = cx.new(|_| ScrubState::default());
        cx.observe(&scrub, |_, _, cx| cx.notify()).detach();
        Self { scrub }
    }

    /// Text describing the currently scrubbed sample, if any.
    fn scrubbed_value(&self, cx: &App) -> Option<SharedString> {
        let state = self.scrub.read(cx);
        state.first().map(|result| {
            format!(
                "value {} (sample {})",
                result.sample.label, result.sample.index
            )
            .into()
        })
    }
}

impl Render for DemoRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scrubbed = self.scrubbed_value(cx);
        div()
            .size_full()
            .bg(rgb(BACKGROUND))
            .p_4()
            .flex()
            .flex_wrap()
            .gap_4()
            .children(examples::all(&self.scrub).into_iter().map(|example| {
                let value = example.live.then(|| scrubbed.clone()).flatten();
                card(example, value)
            }))
    }
}

/// Wraps a graph in a titled card with a fixed size so the graph's layout
/// behaviour inside a parent element is exercised.
fn card(example: Example, value: Option<SharedString>) -> impl IntoElement {
    let header = div()
        .flex()
        .justify_between()
        .items_baseline()
        .text_sm()
        .child(
            div()
                .text_color(rgb(TITLE_COLOR))
                .child(SharedString::from(example.title)),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(if value.is_some() {
                    VALUE_COLOR
                } else {
                    NOTE_COLOR
                }))
                .child(value.unwrap_or_else(|| SharedString::from(example.note))),
        );
    div()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
        .bg(rgb(CARD_BACKGROUND))
        .rounded_md()
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(header)
        .child(div().flex_1().child(example.graph))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(DemoRoot::new),
        )
        .expect("failed to open the demo window");
        cx.activate(true);
    });
}
