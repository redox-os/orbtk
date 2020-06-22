//! self module contains a platform specific implementation of the window shell.

use std::sync::mpsc;

pub use super::native::*;

use glutin::event_loop::{ControlFlow, EventLoop};
use lazy_static;

use crate::prelude::*;

use self::states::*;
pub use self::window::*;
pub use self::window_builder::*;

mod states;
mod window;
mod window_builder;

/// Does nothing. self function is only use by the web backend.
pub fn initialize() {}

/// Represents an application shell that could handle multiple windows.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
    requests: mpsc::Receiver<ShellRequest<A>>,
    event_loop: Vec<EventLoop<()>>,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new(requests: mpsc::Receiver<ShellRequest<A>>) -> Self {
        Shell {
            window_shells: vec![],
            requests,
            event_loop: vec![EventLoop::new()],
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(self, adapter)
    }

    /// Creates a window builder from a settings object.
    pub fn create_window_from_settings(
        &mut self,
        settings: WindowSettings,
        adapter: A,
    ) -> WindowBuilder<A> {
        WindowBuilder::from_settings(settings, self, adapter)
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        let mut requests = vec![];
        for request in self.requests.try_iter() {
            requests.push(request);
        }

        for request in requests {
            match request {
                ShellRequest::CreateWindow(adapter, settings, window_requests) => {
                    self.create_window_from_settings(settings, adapter)
                        .request_receiver(window_requests)
                        .build();
                }
            }
        }
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        self.event_loop.get(0).unwrap()
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(mut self) {
        self.event_loop
            .pop()
            .unwrap()
            .run(move |event, _, control_flow| {
                if self.window_shells.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }

                for i in 0..self.window_shells.len() {
                    let mut remove = false;
                    if let Some(window_shell) = self.window_shells.get_mut(i) {
<<<<<<< HEAD
                        window_shell.drain_events(control_flow, &event);
                        window_shell.receive_requests();
                        window_shell.update();
                        window_shell.render();

=======
                        window_shell.render();
                        window_shell.update();
                        window_shell.drain_events(control_flow, &event);
                        window_shell.receive_requests();
>>>>>>> origin/develop
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
