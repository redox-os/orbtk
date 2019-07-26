use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use glutin::{
    dpi::PhysicalSize, ContextBuilder, ControlFlow, ElementState, Event, EventsLoop, GlProfile,
    GlRequest, KeyboardInput, VirtualKeyCode, WindowBuilder as GlutinWindowBuilder, WindowEvent,
};

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    events: Vec<Event>,
    mouse_position: Point,
    window_size: (f64, f64),
    render_context_2_d: RenderContext2D,
    window_builder_helper: WindowBuilderHelper,
    adapter: A,
    key_state: Option<ButtonState>,
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render context 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self) {
        if let Some(event) = self.events.pop() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        self.window_size = (size.width, size.height);
                        self.render_context_2_d.resize(size.width, size.height);
                        self.adapter.resize(size.width, size.height);
                    }
                    WindowEvent::CloseRequested => self.adapter.quite_event(),
                    WindowEvent::ReceivedCharacter(c) => {
                        if let Some(key_state) = self.key_state {
                            if key_state == ButtonState::Down {
                                self.adapter.key_event(KeyEvent {
                                    key: Key::from(c),
                                    state: ButtonState::Down,
                                    text: c.to_string(),
                                })
                            } else {
                                self.adapter.key_event(KeyEvent {
                                    key: Key::from(c),
                                    state: ButtonState::Up,
                                    text: c.to_string(),
                                })
                            }
                        }

                        self.key_state = None;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                scancode,
                                state,
                                ..
                            },
                        ..
                    } => {
                        let key = {
                            match keycode {
                                VirtualKeyCode::Back => Some(Key::Backspace),
                                VirtualKeyCode::Delete => Some(Key::Delete),
                                VirtualKeyCode::LControl => Some(Key::Control),
                                VirtualKeyCode::LShift => Some(Key::ShiftL),
                                VirtualKeyCode::RShift => Some(Key::ShiftR),
                                VirtualKeyCode::LAlt => Some(Key::Alt),
                                VirtualKeyCode::RAlt => Some(Key::Alt),
                                VirtualKeyCode::Up => Some(Key::Up),
                                VirtualKeyCode::Down => Some(Key::Down),
                                VirtualKeyCode::Left => Some(Key::Left),
                                VirtualKeyCode::Right => Some(Key::Right),
                                _ => None,
                            }
                        };

                        match state {
                            ElementState::Pressed => {
                                if let Some(key) = key {
                                    self.adapter.key_event(KeyEvent {
                                        key,
                                        state: ButtonState::Down,
                                        text: String::from(""),
                                    });
                                } else {
                                    self.key_state = Some(ButtonState::Down);
                                }
                            }
                            _ => {
                                if let Some(key) = key {
                                    self.adapter.key_event(KeyEvent {
                                        key,
                                        state: ButtonState::Up,
                                        text: String::from(""),
                                    });
                                } else {
                                    self.key_state = Some(ButtonState::Up);
                                }
                            }
                        }
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        let button = {
                            match button {
                                glutin::MouseButton::Left => MouseButton::Left,
                                glutin::MouseButton::Right => MouseButton::Right,
                                _ => MouseButton::Middle,
                            }
                        };

                        match state {
                            ElementState::Pressed => {
                                self.adapter.mouse_event(MouseEvent {
                                    x: self.mouse_position.x,
                                    y: self.mouse_position.y,
                                    button,
                                    state: ButtonState::Down,
                                });
                            }
                            _ => {
                                self.adapter.mouse_event(MouseEvent {
                                    x: self.mouse_position.x,
                                    y: self.mouse_position.y,
                                    button,
                                    state: ButtonState::Up,
                                });
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_position.x = position.x;
                        self.mouse_position.y = position.y;
                        self.adapter.mouse(position.x, position.y);
                    }
                    // `CloseRequested` and `KeyboardInput` events won't appear here.
                    _ => (),
                },

                _ => (),
            }
        }
    }
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter + 'static,
{
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub updater: Box<dyn Updater>,
}

impl<A> ShellRunner<A>
where
    A: WindowAdapter,
{
    pub fn run(mut self) {
        let window_builder_helper = self.window_shell.borrow().window_builder_helper.clone();

        let mut events_loop = EventsLoop::new();
        let hidpi_factor = events_loop.get_primary_monitor().get_hidpi_factor();
        let physical_window_size = PhysicalSize::new(
            window_builder_helper.bounds.width,
            window_builder_helper.bounds.height,
        );
        let logical_window_size = physical_window_size.to_logical(hidpi_factor);

        // Creates inner window.
        let window_builder = GlutinWindowBuilder::new()
            .with_title(window_builder_helper.title)
            .with_resizable(window_builder_helper.resizeable)
            .with_dimensions(logical_window_size);

        // Create an OpenGL 3.x context.
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window_builder, &events_loop)
            .unwrap();

        // Load OpenGL, and make the context current.
        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        self.window_shell
            .borrow_mut()
            .render_context_2_d
            .init_renderer();

        events_loop.run_forever(|event| {
            if !self.running.get() {
                return ControlFlow::Break;
            }

            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        let dpi = gl_context.window().get_hidpi_factor();
                        gl_context.resize(logical_size.to_physical(dpi));
                    }
                    _ => (),
                },
                _ => (),
            }

            self.updater.update();

            if self.update.get() {
                self.update.set(false);
                self.window_shell.borrow_mut().render_context_2_d.render();
                gl_context.swap_buffers().unwrap();
            }

            self.window_shell.borrow_mut().events.push(event);

            self.window_shell.borrow_mut().drain_events();
            ControlFlow::Continue
        });
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    resizeable: bool,

    bounds: Rect,

    adapter: A,
}

#[derive(Clone)]
pub struct WindowBuilderHelper {
    title: String,

    resizeable: bool,

    bounds: Rect,
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
            resizeable: false,
            bounds: Rect::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rect>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(self) -> WindowShell<A> {
        WindowShell {
            window_size: (self.bounds.width, self.bounds.height),
            render_context_2_d: RenderContext2D::new(self.bounds.width, self.bounds.height),
            window_builder_helper: WindowBuilderHelper {
                title: self.title,
                bounds: self.bounds,
                resizeable: self.resizeable,
            },
            events: vec![],
            mouse_position: Point::default(),
            adapter: self.adapter,
            key_state: None,
        }
    }
}

pub fn log(message: String) {
    println!("{}", message);
}
