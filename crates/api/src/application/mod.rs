//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::sync::mpsc;

use dces::prelude::Entity;

use crate::{
    prelude::*,
    shell::{Shell, ShellRequest},
};

pub use self::context_provider::*;
pub use self::global::*;
pub use self::overlay::*;
pub use self::window_adapter::*;

mod context_provider;
mod global;
mod overlay;
mod window_adapter;

/// The `Application` represents the entry point of an OrbTk based application.
pub struct Application {
    // shells: Vec<Shell<WindowAdapter>>,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    shell: Shell<WindowAdapter>,
    name: Box<str>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Application::from_name("orbtk_application")
    }

    /// Create a new application with the given name.
    pub fn from_name(name: impl Into<Box<str>>) -> Self {
        let (sender, receiver) = mpsc::channel();

        Application {
            request_sender: sender,
            name: name.into(),
            shell: Shell::new(receiver),
        }
    }

    /// Creates a new window and add it to the application.
    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let (adapter, settings, receiver) = create_window(self.name.clone(), self.request_sender.clone(), create_fn);

        self.shell
            .create_window_from_settings(settings, adapter)
            .request_receiver(receiver)
            .build();

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
        self.shell.run();
    }
}
