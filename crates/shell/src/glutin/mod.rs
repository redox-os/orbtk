use std::sync::mpsc::{channel, Receiver, Sender};

use glutin::dpi::{PhysicalSize, LogicalSize};
use glutin::event::{self, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_resources::embedded::EmbeddedResourceLoader;

pub use super::native::*;

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

/// Concrete implementation of the window shell.
pub struct Shell<A: 'static>
where
    A: ShellAdapter,
{
    adapter: A,
    update: bool,
    render: bool,
    running: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
    render_context_2_d: RenderContext2D,
    window_builder: WindowBuilder,
    mouse_pos: (f64, f64),
    window_size: (f64, f64),
}

impl<A> Shell<A>
where
    A: ShellAdapter,
{
    /// Gets if the shell is running.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Gets a a new sender to send request to the window shell.
    pub fn request_sender(&self) -> Sender<ShellRequest> {
        self.request_sender.clone()
    }

    /// Sets running.
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    /// Get if the shell should be updated.
    pub fn update(&self) -> bool {
        self.update
    }

    /// Sets update.
    pub fn set_update(&mut self, update: bool) {
        self.update = update;
    }
    
    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render ctx 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(s),
                ..
            } => {
                self.adapter.resize(s.width as f64, s.height as f64);
                self.render_context_2_d().resize(s.width as f64, s.height as f64);
                self.update = true;
                *control_flow = ControlFlow::Wait;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                self.adapter.quit_event();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                // todo: implement
                ..
            } => *control_flow = ControlFlow::Wait,
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let button = {
                    match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Right => MouseButton::Right,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Other(_) => MouseButton::Left,
                    }
                };

                let state = {
                    match state {
                        event::ElementState::Pressed => ButtonState::Down,
                        event::ElementState::Released => ButtonState::Up,
                    }
                };

                let mouse_pos = self.mouse_pos;

                self.adapter.mouse_event(MouseEvent {
                    x: mouse_pos.0,
                    y: mouse_pos.1,
                    button,
                    state,
                });
                self.update = true;
                self.render = true;
                *control_flow = ControlFlow::Wait;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                match delta {
                    event::MouseScrollDelta::LineDelta(_, _) => {}
                    event::MouseScrollDelta::PixelDelta(p) => {
                        self.adapter.scroll(p.x, p.y);
                    }
                }
                self.render = true;
                self.update = true;
                *control_flow = ControlFlow::Wait;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.mouse_pos = (position.x, position.y);
                self.adapter.mouse(position.x, position.y);
                self.update = true;
                self.render = true;
                *control_flow = ControlFlow::Wait;
            }
            _ => *control_flow = ControlFlow::Wait
        }
    }

    pub fn run(mut self) {
        // Open a window.
        // Calculate the right logical size of the window.
        let event_loop = EventLoop::new();
        let window_size = vec2i(self.window_size.0 as i32, self.window_size.1 as i32);

        // Create an OpenGL 3.x context for Pathfinder to use.
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(self.window_builder.clone(), &event_loop)
            .unwrap();

        // Load OpenGL, and make the context current.
        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        // Create a Pathfinder renderer.
        let mut renderer = Renderer::new(
            GLDevice::new(GLVersion::GL3, 0),
            &EmbeddedResourceLoader::new(),
            DestFramebuffer::full_window(window_size),
            RendererOptions {
                background_color: Some(ColorF::white()),
                ..RendererOptions::default()
            },
        );

        self.render_context_2_d =
            RenderContext2D::new_ex(self.window_size, renderer);

        // Wait for a keypress.
        event_loop.run(move |event, _, control_flow| {
            self.drain_events(event, control_flow);

            if self.update {
                self.adapter.run(&mut self.render_context_2_d);
                self.update = false;
            }

            if self.render {
                gl_context.swap_buffers().unwrap();
                self.render = false;
            }
        });
    }
}

/// Constructs the window shell
pub struct ShellBuilder<A>
where
    A: ShellAdapter,
{
    title: String,

    borderless: bool,

    resizeable: bool,

    always_on_top: bool,

    bounds: Rectangle,

    adapter: A,
}

impl<A> ShellBuilder<A>
where
    A: ShellAdapter,
{
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        ShellBuilder {
            adapter,
            title: String::default(),
            borderless: false,
            resizeable: false,
            always_on_top: false,
            bounds: Rectangle::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Does nothing on web.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(self) -> Shell<A> {
        let (request_sender, request_receiver) = channel();

        // Calculate the right logical size of the window.
        // let event_loop = EventLoop::new();
        let window_size = (self.bounds.width(), self.bounds.height());

        let logical_size = PhysicalSize::new(window_size.0, window_size.1);
        // Open a window.
        let window_builder = WindowBuilder::new()
            .with_inner_size(logical_size)
            .with_title(self.title)
            .with_resizable(self.resizeable)
            .with_always_on_top(self.always_on_top)
            .with_decorations(!self.borderless);

        Shell {
            render: true,
            update: true,
            running: true,
            request_receiver,
            request_sender,
            render_context_2_d: RenderContext2D::new(window_size.0, window_size.1),
            adapter: self.adapter,
            window_builder,
            mouse_pos: (0.0, 0.0),
            window_size
        }
    }
}
