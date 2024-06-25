use vulkanalia_demo::application::Application;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = Application::default();
    event_loop.run_app(&mut app).map_err(Into::into)
}
