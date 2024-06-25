use crate::vulkan::create_vulkan_resources;
use vk::ExtDebugUtilsExtension;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::{Entry, Instance};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct ApplicationData {
    pub messenger: vk::DebugUtilsMessengerEXT,
}

#[derive(Default)]
pub struct Application {
    window: Option<Window>,
    entry: Option<Entry>,
    instance: Option<Instance>,
    data: ApplicationData,
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
        let mut data = ApplicationData::default();
        let (entry, instance) =
            unsafe { create_vulkan_resources(self.window.as_ref().unwrap(), &mut data).unwrap() };
        self.entry = Some(entry);
        self.instance = Some(instance);
        self.data = data;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                unsafe {
                    self.instance
                        .as_ref()
                        .unwrap()
                        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
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
