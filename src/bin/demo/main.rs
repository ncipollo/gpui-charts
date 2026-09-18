//! A demo app for visually testing gpui-charts components.
//!
//! Every plot type and builder in the library is exercised here. No library
//! logic lives in this binary; it only calls the public API.

mod examples;

use gpui::{
    App, AppContext, Application, Bounds, Context, IntoElement, Render, SharedString, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_charts::Graph;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 900.0;
const CARD_WIDTH: f32 = 400.0;
const CARD_HEIGHT: f32 = 270.0;

const BACKGROUND: u32 = 0x1c1c1e;
const CARD_BACKGROUND: u32 = 0x2a2a2d;
const TITLE_COLOR: u32 = 0xe6e6e6;

struct DemoRoot;

impl Render for DemoRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(BACKGROUND))
            .p_4()
            .flex()
            .flex_wrap()
            .gap_4()
            .children(
                examples::all()
                    .into_iter()
                    .map(|(title, graph)| card(title, graph)),
            )
    }
}

/// Wraps a graph in a titled card with a fixed size so the graph's layout
/// behaviour inside a parent element is exercised.
fn card(title: &'static str, graph: Graph) -> impl IntoElement {
    div()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
        .bg(rgb(CARD_BACKGROUND))
        .rounded_md()
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_color(rgb(TITLE_COLOR))
                .text_sm()
                .child(SharedString::from(title)),
        )
        .child(div().flex_1().child(graph))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| DemoRoot),
        )
        .expect("failed to open the demo window");
        cx.activate(true);
    });
}
