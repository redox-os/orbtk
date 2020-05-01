use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder as WinitWindowBuilder,
};

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};

pub use super::native::*;

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    pub flip: bool,
    adapter: A,
    update: bool,
    running: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
    render_context_2_d: RenderContext2D,
    window_builder: WinitWindowBuilder,
}

// unsafe impl<A> HasRawWindowHandle for WindowShell<A>
// where
//     A: WindowAdapter,
// {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//         // let handle = WebHandle {
//         //     id: 0,
//         //     ..WebHandle::empty()
//         // };

//         // RawWindowHandle::N
//     }
// }

impl<A> WindowShell<A>
where
    A: WindowAdapter,
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
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter + 'static,
{
    pub shell: Rc<RefCell<WindowShell<A>>>,
    pub updater: Box<dyn Updater>,
}

impl<A> ShellRunner<A>
where
    A: WindowAdapter,
{
    pub fn run(mut self) {
        let event_loop = EventLoop::new();

        let window = self
            .shell
            .borrow_mut()
            .window_builder()
            .clone()
            .build(&event_loop)
            .unwrap();
        


        let mut pixels = {
            let surface = Surface::create(&window);
            let surface_texture = SurfaceTexture::new(486, 730, surface);
            Pixels::new(486, 730, surface_texture).unwrap()
        };

        event_loop.run(move |event, _, control_flow| {
            self.updater.update();
            self.shell.borrow_mut().set_update(false);

            if let Some(data) = self.shell.borrow_mut().render_context_2_d.data() {
                pixels.get_frame().copy_from_slice(data);
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                }
                | Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    *control_flow = ControlFlow::Wait;
                }
            };
        })
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    borderless: bool,

    resizeable: bool,

    always_on_top: bool,

    bounds: Rectangle,

    adapter: A,
}

impl<A> WindowBuilder<A>
where
    A: WindowAdapter,
{
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        WindowBuilder {
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
    pub fn build(self) -> WindowShell<A> {
        let (request_sender, request_receiver) = channel();

        // Calculate the right logical size of the window.
        // let event_loop = EventLoop::new();
        let size = (self.bounds.width(), self.bounds.height());

        let physical_window_size = PhysicalSize::new(size.0, size.1);
        // Open a window.
        let window_builder = WinitWindowBuilder::new()
            .with_title(self.title)
            .with_resizable(self.resizeable)
            .with_always_on_top(self.always_on_top)
            .with_decorations(!self.borderless)
            .with_inner_size(physical_window_size);

        // // Create an OpenGL 3.x context for Pathfinder to use.
        // let gl_context = ContextBuilder::new()
        //     .with_gl(GlRequest::Latest)
        //     .with_gl_profile(GlProfile::Core)
        //     .build_windowed(window_builder, &event_loop)
        //     .unwrap();

        // let window_thread = thread::spawn(move || {
        //     event_loop.run(move |event, _, control_flow| {
        //         match event {
        //             Event::WindowEvent { event: WindowEvent::CloseRequested, .. } |
        //             Event::WindowEvent {
        //                 event: WindowEvent::KeyboardInput {
        //                     input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. },
        //                     ..
        //                 },
        //                 ..
        //             } => {
        //                 *control_flow = ControlFlow::Exit;
        //             },
        //             _ => {
        //                 *control_flow = ControlFlow::Wait;
        //             },
        //         };
        //     })
        // });

        // // Load OpenGL, and make the context current.
        // let gl_context = unsafe { gl_context.make_current().unwrap() };
        // gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        WindowShell {
            flip: false,
            update: true,
            running: true,
            request_receiver,
            request_sender,
            render_context_2_d: RenderContext2D::new(size.0, size.1),
            adapter: self.adapter,
            window_builder,
        }
    }
}
