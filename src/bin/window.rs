use anyhow::Result;
use support::{run_app, App};

#[derive(Default)]
struct DemoApp;

impl App for DemoApp {}

fn main() -> Result<()> {
    run_app(DemoApp::default(), "Window")
}
