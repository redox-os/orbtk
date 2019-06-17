use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use glutin::{
    dpi::PhysicalSize, ContextBuilder, ControlFlow, ElementState, Event, EventsLoop, GlProfile,
    GlRequest, KeyboardInput, VirtualKeyCode, WindowBuilder as GlutinWindowBuilder, WindowEvent,
};

use crate::{obsolete, prelude::*, utils::*};

pub fn initialize() {}

pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    events: Vec<Event>,
    mouse_position: Point,
    window_builder_helper: WindowBuilderHelper,
    adapter: A,
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    fn drain_events(&mut self) {
        if let Some(event) = self.events.pop() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => self.adapter.quite_event(),
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
                                VirtualKeyCode::Back => Key::Backspace,
                                VirtualKeyCode::Delete => Key::Delete,
                                VirtualKeyCode::LControl => Key::Control,
                                VirtualKeyCode::LShift => Key::ShiftL,
                                VirtualKeyCode::RShift => Key::ShiftR,
                                VirtualKeyCode::LAlt => Key::Alt,
                                VirtualKeyCode::RAlt => Key::Alt,
                                VirtualKeyCode::Up => Key::Up,
                                VirtualKeyCode::Down => Key::Down,
                                VirtualKeyCode::Left => Key::Left,
                                VirtualKeyCode::Right => Key::Right,
                                _ => match std::char::from_u32(scancode).unwrap() {
                                    '\n' => Key::Enter,
                                    _ => Key::from(std::char::from_u32(scancode).unwrap()),
                                },
                            }
                        };

                        match state {
                            ElementState::Pressed => {
                                self.adapter.key_event(KeyEvent {
                                    key,
                                    state: ButtonState::Down,
                                });
                            }
                            _ => {
                                self.adapter.key_event(KeyEvent {
                                    key,
                                    state: ButtonState::Down,
                                });
                            }
                        }
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        let button = {
                            match button {
                                glutin::MouseButton::Right => MouseButton::Left,
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
                    WindowEvent::Moved(pos) => {
                        self.mouse_position.x = pos.x;
                        self.mouse_position.y = pos.y;
                        self.adapter.mouse(pos.x, pos.y);
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
        // gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        // gl_context.swap_buffers().unwrap();

        events_loop.run_forever(|event| {
            if !self.running.get() {
                return ControlFlow::Break;
            }

            self.updater.update();

            self.update.set(false);

            self.window_shell.borrow_mut().events.push(event);
            // match event {

            //     Event::WindowEvent {
            //         event: WindowEvent::CloseRequested,
            //         ..
            //     } => {
            //         println!("The close button was pressed; stopping");
            //         self.running.set(false);
            //     }
            //     _ => ()
            // }

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
            window_builder_helper: WindowBuilderHelper {
                title: self.title,
                bounds: self.bounds,
                resizeable: self.resizeable,
            },
            events: vec![],
            mouse_position: Point::default(),
            adapter: self.adapter,
        }
    }
}

pub fn log(message: String) {
    println!("{}", message);
}
