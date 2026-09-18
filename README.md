# gpui-charts

Chart components for [gpui](https://www.gpui.rs/), Zed's GPU-accelerated UI framework.

## Requirements

- Stable Rust (edition 2024)
- macOS with Xcode installed and selected (`sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`) — gpui renders with Metal

## Demo

    cargo run --bin demo

Opens a window for visually testing chart components as they're built.

## Develop

    cargo fmt
    cargo test
    cargo clippy
