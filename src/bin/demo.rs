//! A demo app for visually testing gpui-charts components.

use gpui::{
    App, AppContext, Application, Bounds, Context, IntoElement, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};
use gpui_charts::{AxesBuilder, GraphBuilder};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

struct DemoRoot;

impl Render for DemoRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let axes = AxesBuilder::new().build().expect("valid axes");
        let graph = GraphBuilder::new().axes(axes).build();
        div().size_full().child(graph)
    }
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
