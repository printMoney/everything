//! BlackWhite: a window that is fully black or fully white.
//! Click anywhere to switch. No network, no files, nothing else.

use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

type Surface = softbuffer::Surface<Rc<Window>, Rc<Window>>;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface>,
    white: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("BlackWhite")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        let window = Rc::new(event_loop.create_window(attrs).expect("create window"));
        let context = softbuffer::Context::new(window.clone()).expect("softbuffer context");
        let surface = Surface::new(&context, window.clone()).expect("softbuffer surface");
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput { state: ElementState::Pressed, .. } => {
                self.white = !self.white;
                window.request_redraw();
            }
            WindowEvent::Resized(_) => window.request_redraw(),
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();
                let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                else {
                    return;
                };
                surface.resize(w, h).expect("resize");
                let mut buffer = surface.buffer_mut().expect("buffer");
                buffer.fill(if self.white { 0x00FF_FFFF } else { 0x0000_0000 });
                buffer.present().expect("present");
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.run_app(&mut App::default()).expect("run");
}
