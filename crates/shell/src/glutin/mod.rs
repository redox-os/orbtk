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

impl<A> Shell<A>
where
    A: WindowAdapter,
{
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