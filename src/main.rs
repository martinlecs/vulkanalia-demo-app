use vulkanalia::prelude::v1_0::*;
use vulkanalia::{Entry, Instance};
use vulkanalia_demo::vulkan::create_vulkan_resources;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct Application {
    window: Option<Window>,
    entry: Option<Entry>,
    instance: Option<Instance>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Vulkania-demo app")
                        .with_inner_size(LogicalSize::new(1024, 768)),
                )
                .unwrap(),
        );
        let (entry, instance) =
            unsafe { create_vulkan_resources(self.window.as_ref().unwrap()).unwrap() };
        self.entry = Some(entry);
        self.instance = Some(instance);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                unsafe {
                    self.instance.as_ref().unwrap().destroy_instance(None);
                };
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = Application::default();
    event_loop.run_app(&mut app).map_err(Into::into)
}
