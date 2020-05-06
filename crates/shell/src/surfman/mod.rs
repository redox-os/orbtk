use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use euclid::default::Size2D;
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion as SurfmanGLVersion,
};
use surfman::{SurfaceAccess, SurfaceType};
use winit::dpi::LogicalSize;
use winit::{ControlFlow, Event, EventsLoop, WindowBuilder as WinitWindowBuilder, WindowEvent};

pub use super::native::*;

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

use std::marker::PhantomData;

/// Concrete implementation of the window shell.
pub struct Shell<A>
where
    A: ShellAdapter,
{
    flip: bool,
    adapter: A,
    update: bool,
    running: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
    render_context_2_d: RenderContext2D,
    window_builder: WinitWindowBuilder,
    mouse_pos: (f64, f64),
}

// unsafe impl<A> HasRawWindowHandle for Shell<A>
// where
//     A: ShellAdapter,
// {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//         // let handle = WebHandle {
//         //     id: 0,
//         //     ..WebHandle::empty()
//         // };

//         // RawWindowHandle::N
//     }
// }

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

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, red: u8, green: u8, blue: u8) {}

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render ctx 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self) {}

    pub fn flip(&mut self) {
        self.flip = false;
    }

    fn window_builder(&self) -> &WinitWindowBuilder {
        &self.window_builder
    }

    pub fn run(&mut self) {
        // Open a window.
        let mut event_loop = EventsLoop::new();
        let size = self
            .window_builder
            .window
            .dimensions
            .unwrap_or(LogicalSize::new(100.0, 100.0));

        let mut render = true;

        let logical_size = LogicalSize::new(size.width as f64, size.height as f64);

        let window = self.window_builder.clone().build(&event_loop).unwrap();
        window.show();

        // Create a `surfman` device. On a multi-GPU system, we'll request the low-power integrated
        // GPU.
        let connection = Connection::from_winit_window(&window).unwrap();
        let native_widget = connection
            .create_native_widget_from_winit_window(&window)
            .unwrap();
        let adapter = connection.create_low_power_adapter().unwrap();
        let mut device = connection.create_device(&adapter).unwrap();

        // Request an OpenGL 3.x context. Pathfinder requires this.
        let context_attributes = ContextAttributes {
            version: SurfmanGLVersion::new(3, 0),
            flags: ContextAttributeFlags::ALPHA,
        };
        let context_descriptor = device
            .create_context_descriptor(&context_attributes)
            .unwrap();

        // Make the OpenGL context via `surfman`, and load OpenGL functions.
        let surface_type = SurfaceType::Widget { native_widget };
        let mut context = device.create_context(&context_descriptor).unwrap();
        let surface = device
            .create_surface(&context, SurfaceAccess::GPUOnly, surface_type)
            .unwrap();
        device
            .bind_surface_to_context(&mut context, surface)
            .unwrap();
        device.make_context_current(&context).unwrap();
        gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

        // Get the real size of the window, taking HiDPI into account.
        let hidpi_factor = window.get_current_monitor().get_hidpi_factor();
        let physical_size = logical_size.to_physical(hidpi_factor);
        let framebuffer_size = vec2i(physical_size.width as i32, physical_size.height as i32);

        // Create a Pathfinder GL device.
        let default_framebuffer = device
            .context_surface_info(&context)
            .unwrap()
            .unwrap()
            .framebuffer_object;
        let pathfinder_device = GLDevice::new(GLVersion::GL3, default_framebuffer);

        // Create a Pathfinder renderer.
        let mut renderer = Renderer::new(
            pathfinder_device,
            &EmbeddedResourceLoader::new(),
            DestFramebuffer::full_window(framebuffer_size),
            RendererOptions {
                background_color: Some(ColorF::white()),
                ..RendererOptions::default()
            },
        );

        self.render_context_2_d =
            RenderContext2D::new_ex((size.width as f64, size.height as f64), renderer);

        // Wait for a keypress.
        event_loop.run_forever(|evt| match evt {
            Event::WindowEvent {
                event: WindowEvent::Resized(s),
                ..
            } => {
                self.adapter.resize(s.width, s.height);
                ControlFlow::Continue
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                self.adapter.quit_event();
                ControlFlow::Break
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => ControlFlow::Continue,
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let button = {
                    match button {
                        winit::MouseButton::Left => MouseButton::Left,
                        winit::MouseButton::Right => MouseButton::Right,
                        winit::MouseButton::Middle => MouseButton::Middle,
                        winit::MouseButton::Other(_) => MouseButton::Left,
                    }
                };

                let state = {
                    match state {
                        winit::ElementState::Pressed => ButtonState::Down,
                        winit::ElementState::Released => ButtonState::Up,
                    }
                };

                let mouse_pos = self.mouse_pos;

                self.adapter.mouse_event(MouseEvent {
                    x: mouse_pos.0,
                    y: mouse_pos.1,
                    button,
                    state,
                });
                render = true;
                ControlFlow::Continue
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                match delta {
                    winit::MouseScrollDelta::LineDelta(_, _) => {}
                    winit::MouseScrollDelta::PixelDelta(p) => {
                        self.adapter.scroll(p.x, p.y);
                    }
                }
                ControlFlow::Continue
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.mouse_pos = (position.x, position.y);
                self.adapter.mouse(position.x, position.y);
                render = true;
                ControlFlow::Continue
            }
            // | Event::WindowEvent {
            //     event: WindowEvent::KeyboardInput { .. },
            //     ..
            // } => ControlFlow::Break,
            _ => {
                // if let Some(updater) = &mut self.updater {
                //     updater.update();
                // }
                // todo: shell context
                self.adapter.run(&mut self.render_context_2_d);
                self.set_update(true);
                self.flip();
                self.drain_events();

                if render {
                    // Present the rendered canvas via `surfman`.
                    let mut surface = device
                        .unbind_surface_from_context(&mut context)
                        .unwrap()
                        .unwrap();
                    device.present_surface(&mut context, &mut surface).unwrap();
                    device
                        .bind_surface_to_context(&mut context, surface)
                        .unwrap();
                    render = false;
                }

                ControlFlow::Continue
            }
        });

        // Clean up.
        drop(device.destroy_context(&mut context));
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
        let size = (self.bounds.width(), self.bounds.height());

        let logical_size = LogicalSize::new(size.0, size.1);
        // Open a window.
        let window_builder = WinitWindowBuilder::new()
            .with_dimensions(logical_size)
            .with_title(self.title)
            .with_resizable(self.resizeable)
            .with_always_on_top(self.always_on_top)
            .with_decorations(!self.borderless);

        Shell {
            flip: false,
            update: true,
            running: true,
            request_receiver,
            request_sender,
            render_context_2_d: RenderContext2D::new(size.0, size.1),
            adapter: self.adapter,
            window_builder,
            mouse_pos: (0.0, 0.0),
        }
    }
}
