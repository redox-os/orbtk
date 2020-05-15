//! This module contains a platform specific implementation of the window shell.

use std::sync::mpsc;

pub use super::native::*;

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
/// is based on `minifb`.
pub struct Shell<A>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
    requests: mpsc::Receiver<ShellRequest<A>>
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new(requests: mpsc::Receiver<ShellRequest<A>>) -> Self {
        Shell {
            window_shells: vec![],
            requests
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(
            self,
            adapter
        )
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(&mut self) {
        loop {
            if self.window_shells.is_empty() {
                return;
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

            if let Ok(_request) = self.requests.try_recv() {
                println!("Rec");
            }
        }
    }
}
