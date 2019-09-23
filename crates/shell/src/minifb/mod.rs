//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use minifb;

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    window: minifb::Window,
    render_context_2_d: RenderContext2D,
    adapter: A,
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Creates a new window shell with an adapter.
    pub fn new(window: minifb::Window, adapter: A) -> WindowShell<A> {
        let render_context_2_d = RenderContext2D::new();

        WindowShell {
            window,
            // window_size: window_size,
            // mouse_buttons: (false, false, false),
            // mouse_position: Point::default(),
            render_context_2_d,
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

    fn drain_events(&mut self) {}

    pub fn flip(&mut self) {}
}

impl<A> Drop for WindowShell<A>
where
    A: WindowAdapter,
{
    fn drop(&mut self) {}
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
            if !self.running.get() || !self.window_shell.borrow().window.is_open()  {
                break;
            }

            self.updater.update();

            if self.update.get() {
                self.update.set(false);
                // self.window_shell
                //     .borrow_mut()
                //     .render_context_2_d
                //     .window
                //     .sync();
            }

            self.window_shell.borrow_mut().drain_events();
            self.window_shell.borrow_mut().window.update();
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
        let mut window = minifb::Window::new(
            self.title.as_str(),
            self.bounds.width as usize,
            self.bounds.height as usize,
            minifb::WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        // let mut flags = vec![];
        if self.resizeable {
            // flags.push(WindowFlag::Resizable);
        }

        WindowShell::new(
            window,
            // Window::new_flags(
            //     self.bounds.x as i32,
            //     self.bounds.y as i32,
            //     self.bounds.width as u32,
            //     self.bounds.height as u32,
            //     &self.title,
            //     &flags,
            // )
            // .unwrap(),
            self.adapter,
        )
    }
}

pub fn log(message: String) {
    println!("{}", message);
}
