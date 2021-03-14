use anyhow::Result;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window, window::WindowBuilder};

pub trait App {}

pub fn run_app(app: impl App, title: &str) -> Result<()> {
    let (event_loop, window) = create_window(title)?;
    Ok(())
}

fn create_window(title: &str) -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(title.to_string())
        .with_inner_size(PhysicalSize::new(800, 600));

    let window = window_builder.build(&event_loop)?;

    Ok((event_loop, window))
}
