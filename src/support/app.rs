use anyhow::{Context, Result};
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub const LOG_FILE: &str = "application.log";

pub trait App {
    fn on_key(&mut self, _keystate: ElementState, _keycode: VirtualKeyCode) -> Result<()> {
        Ok(())
    }

    fn on_mouse(&mut self, _: MouseButton, _: ElementState) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, _: Event<()>) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn run_app(mut app: impl App + 'static, title: &str) -> Result<()> {
    create_logger()?;

    let (event_loop, window) = create_window(title)?;

    event_loop.run(move |event, _, control_flow| {
        let mut result = || -> Result<()> {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    app.update()?;
                    app.render()?;
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    if (keycode, state) == (VirtualKeyCode::Escape, ElementState::Pressed) {
                        *control_flow = ControlFlow::Exit;
                    }
                    app.on_key(state, keycode)?;
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => {
                    app.on_mouse(button, state)?;
                }
                Event::LoopDestroyed => {
                    app.cleanup()?;
                }
                _ => {}
            }

            Ok(())
        };

        if let Err(error) = result() {
            log::error!("Application Error: {}", error);
        }
    });
}

pub fn create_logger() -> Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::max(),
            Config::default(),
            File::create(LOG_FILE).context(format!(
                "Failed to create log file named: {}",
                LOG_FILE.to_string()
            ))?,
        ),
    ])?;
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
