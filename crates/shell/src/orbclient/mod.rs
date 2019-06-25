//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

use orbclient::{Color, Renderer, Window, WindowFlag};

use crate::{prelude::*,render::*, utils::*};

pub mod fonts;

pub fn initialize() {}

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    pub inner: Window,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    render_context_2_d: RenderContext2D,
    adapter: A,
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Creates a new window shell with an adapter.
    pub fn new(inner: Window, adapter: A) -> WindowShell<A> {
        WindowShell {
            inner,
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
            render_context_2_d: RenderContext2D::new(),
            adapter,
        }
    }

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render context 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self) {
        self.inner.sync();

        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Mouse(event) => {
                    self.mouse_position.x = event.x as f64;
                    self.mouse_position.y = event.y as f64;
                    self.adapter.mouse(event.x as f64, event.y as f64);
                }
                orbclient::EventOption::Button(button) => {
                    if !button.left && !button.middle && !button.right {
                        let button = {
                            if self.mouse_buttons.0 {
                                MouseButton::Left
                            } else if self.mouse_buttons.1 {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };

                        self.adapter.mouse_event(MouseEvent {
                            x: self.mouse_position.x,
                            y: self.mouse_position.y,
                            button,
                            state: ButtonState::Up,
                        });
                    } else {
                        let button = {
                            if button.left {
                                MouseButton::Left
                            } else if button.middle {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };

                        self.adapter.mouse_event(MouseEvent {
                            x: self.mouse_position.x,
                            y: self.mouse_position.y,
                            button,
                            state: ButtonState::Down,
                        });
                    }

                    self.mouse_buttons = (button.left, button.middle, button.right);
                }
                orbclient::EventOption::Key(key_event) => {
                    let key = {
                        match key_event.scancode {
                            orbclient::K_BKSP => Key::Backspace,
                            orbclient::K_DEL => Key::Delete,
                            orbclient::K_CTRL => Key::Control,
                            orbclient::K_LEFT_SHIFT => Key::ShiftL,
                            orbclient::K_RIGHT_SHIFT => Key::ShiftR,
                            orbclient::K_ALT => Key::Alt,
                            orbclient::K_UP => Key::Up,
                            orbclient::K_DOWN => Key::Down,
                            orbclient::K_LEFT => Key::Left,
                            orbclient::K_RIGHT => Key::Right,
                            _ => match key_event.character {
                                '\n' => Key::Enter,
                                _ => Key::from(key_event.character),
                            },
                        }
                    };

                    if key_event.pressed {
                        self.adapter.key_event(KeyEvent {
                            key,
                            state: ButtonState::Up,
                        });
                    } else {
                        self.adapter.key_event(KeyEvent {
                            key,
                            state: ButtonState::Down,
                        });
                    }
                }
                orbclient::EventOption::Quit(_quit_event) => {
                    self.adapter.quite_event();
                }
                orbclient::EventOption::Resize(event) => {
                    self.adapter.resize(event.width as f64, event.height as f64);
                }
                _ => {}
            }
        }
    }
}

impl<A> Drop for WindowShell<A>
where
    A: WindowAdapter,
{
    fn drop(&mut self) {
        self.inner.sync();
    }
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter,
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
    pub fn run(&mut self) {
        loop {
            if !self.running.get() {
                break;
            }

            self.updater.update();

            self.update.set(false);

            self.window_shell.borrow_mut().drain_events();
        }
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
        let mut flags = vec![];
        if self.resizeable {
            flags.push(WindowFlag::Resizable);
        }

        WindowShell::new(
            Window::new_flags(
                self.bounds.x as i32,
                self.bounds.y as i32,
                self.bounds.width as u32,
                self.bounds.height as u32,
                &self.title,
                &flags,
            )
            .unwrap(),
            self.adapter,
        )
    }
}

pub fn log(message: String) {
    println!("{}", message);
}
