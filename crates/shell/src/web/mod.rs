//! This module contains a platform specific implementation of the window shell.

use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc, sync::Arc};


use orbgl_api::Canvas;

use orbtk_utils::{Point, Rect};

use crate::{obsolete, prelude::*};

/// Concrete implementation of the window shell.
pub struct WindowShell<A> where A: WindowAdapter {
  
}

impl<A> WindowShell<A> where A: WindowAdapter {
    /// Creates a new window shell with an adapter.
    pub fn new(adapter: A) -> WindowShell<A> {
        

        WindowShell {
            adapter,
        }
    }

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    fn drain_events(&mut self) {
    }
}

// impl<A> Drop for WindowShell<A> where A: WindowAdapter {
//     fn drop(&mut self) {
//         self.inner.sync();
//     }
// }

/// Implementation of the OrbClient based backend runner.
pub struct ShellRunner<A> where A: WindowAdapter {
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub updater: Box<Updater>,
}

impl<A> ShellRunner<A> where A: WindowAdapter {
    pub fn run(&mut self) {
        // loop {
        //     if !self.running.get() {
        //         break;
        //     }

        //     self.updater.update();

        //     self.update.set(false);

        //     self.window_shell.borrow_mut().drain_events();
        // }
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A> where A: WindowAdapter {
    title: String,

    resizeable: bool,

    bounds: Rect,

    adapter: A,
}

impl<A> WindowBuilder<A> where A: WindowAdapter {
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
            // Window::new_flags(
            //     self.bounds.x as i32,
            //     self.bounds.y as i32,
            //     self.bounds.width as u32,
            //     self.bounds.height as u32,
            //     &self.title,
            //     &flags,
            // ).unwrap(),
            self.adapter,
        )
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---


// --- obsolete will be removed after OrbGL supports text rendering ---