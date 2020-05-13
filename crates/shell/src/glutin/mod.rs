//! This module contains a platform specific implementation of the window shell.

pub use super::native::*;

use glutin::event_loop::{ControlFlow, EventLoop};

use crate::prelude::*;

use self::states::*;
pub use self::window::*;
pub use self::window_builder::*;

mod states;
mod window;
mod window_builder;

/// Does nothing. This function is only use by the web backend.
pub fn initialize() {}

/// Represents an application shell that could handle multiple windows. This implementation
/// is based on `glutin`.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
    event_loop: EventLoop<()>,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new() -> Self {
        Shell {
            window_shells: vec![],
            event_loop: EventLoop::new()
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(
            self,
            adapter
        )
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(&mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            if self.window_shells.is_empty() {
                *control_flow = ControlFlow::Exit;
            }

            for i in 0..self.window_shells.len() {
                let mut remove = false;
                if let Some(window_shell) = self.window_shells.get_mut(i) {
                    window_shell.render();
                    window_shell.update();
                    window_shell.drain_events();
                    window_shell.receive_requests();
                   
                    if !window_shell.is_open() {
                        remove = true;
                    }
                }

                if remove {
                    self.window_shells.remove(i);
                    break;
                }
            }
        });
    }
}
