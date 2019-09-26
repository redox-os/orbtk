//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use minifb;

use spin_sleep::LoopHelper;

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
    mouse_pos: (f32, f32),
    button_down: (bool, bool, bool),
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Creates a new window shell with an adapter.
    pub fn new(window: minifb::Window, adapter: A) -> WindowShell<A> {
        let size = window.get_size();
        let render_context_2_d = RenderContext2D::new(size.0 as f64, size.1 as f64);

        WindowShell {
            window,
            // window_size: window_size,
            // mouse_buttons: (false, false, false),
            // mouse_position: Point::default(),
            render_context_2_d,
            adapter,
            mouse_pos: (0.0, 0.0),
            button_down: (false, false, false),
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
        if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
            if (pos != self.mouse_pos) {
                self.adapter.mouse(pos.0 as f64, pos.1 as f64);
                self.mouse_pos = pos;
            }
        }

        let left_button_down = self.window.get_mouse_down(minifb::MouseButton::Left);

        if left_button_down != self.button_down.0 {
            if left_button_down {
                self.adapter.mouse_event(MouseEvent {
                    x: self.mouse_pos.0 as f64,
                    y: self.mouse_pos.1 as f64,
                    button: MouseButton::Left,
                    state: ButtonState::Down,
                });
            } else {
                self.adapter.mouse_event(MouseEvent {
                    x: self.mouse_pos.0 as f64,
                    y: self.mouse_pos.1 as f64,
                    button: MouseButton::Left,
                    state: ButtonState::Up,
                });
            }
            self.button_down.0 = left_button_down;
        }
    }

    pub fn flip(&mut self) {
        self.window
            .update_with_buffer(self.render_context_2_d.data())
            .unwrap();
    }
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
        let mut loop_helper = LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(60.0);

        // let mut current_fps = None;
        let mut skip = false;

        loop {
            if !self.running.get() || !self.window_shell.borrow().window.is_open() {
                break;
            }

            // let delta = loop_helper.loop_start();

            // if let Some(fps) = loop_helper.report_rate() {
            //     current_fps = Some(fps);
            //     // println!("fps: {}", fps);
            // }

            self.updater.update();

            if self.update.get() {
                self.update.set(false);
                // self.window_shell
                //     .borrow_mut()
                //     .render_context_2_d
                //     .window
                //     .sync();

                skip = true;
            }

            self.window_shell.borrow_mut().drain_events();

           
            self.window_shell.borrow_mut().flip();
            // loop_helper.loop_sleep();
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
