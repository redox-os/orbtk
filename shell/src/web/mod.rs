//! This module contains a platform specific implementation of the window shell.
use std::sync::mpsc;
use stdweb::web::window;

use crate::prelude::*;

use self::states::*;
pub use self::window::*;
pub use self::window_builder::*;

mod states;
mod window;
mod window_builder;

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Initializes web stuff.
pub fn initialize() {
    set_panic_hook();
    stdweb::initialize();
}

/// Represents an application shell that could handle multiple windows.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
    requests: mpsc::Receiver<ShellRequest<A>>,
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
                _ => {}
            }
        }
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(mut self) {
        window().request_animation_frame(move |_| {
            if self.window_shells.is_empty() {
                return;
            }

            for i in 0..self.window_shells.len() {
                let mut remove = false;
                if let Some(window_shell) = self.window_shells.get_mut(i) {
                    window_shell.render();
                    window_shell.update();
                    window_shell.update_clipboard();
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

            self.receive_requests();
            self.run();
        });
    }
}

lazy_static! {
    pub static ref CONSOLE: Console = Console;
}

pub struct Console;

impl Console {
    pub fn time(&self, _name: impl Into<String>) {
        // js! {
        //     console.time(@{&name.into()})
        // }
    }

    pub fn time_end(&self, _name: impl Into<String>) {
        // js! {
        //     console.timeEnd(@{&name.into()})
        // }
    }

    pub fn log(&self, _message: impl Into<String>) {
        #[cfg(feature = "log")]
        js! {
            console.log(@{&_message.into()});
        }
    }
}
