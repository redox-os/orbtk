//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::sync::mpsc;

use dces::prelude::Entity;

use crate::{
    core::{application::WindowAdapter, *},
    localization::*,
    shell::{Shell, ShellRequest},
};

/// The `Application` represents the entry point of an OrbTk based application.
pub struct Application {
    // shells: Vec<Shell<WindowAdapter>>,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    shell: Shell<WindowAdapter>,
    name: Box<str>,
    theme: Rc<Theme>,
    localization: Option<Rc<RefCell<Box<dyn Localization>>>>,
}

impl Default for Application {
    fn default() -> Self {
        Application::from_name("orbtk_application")
    }
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default theme for the application. Could be changed per window.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Rc::new(theme);
        self
    }

    pub fn localization<L>(mut self, localization: L) -> Self
    where
        L: Localization + 'static,
    {
        self.localization = Some(Rc::new(RefCell::new(Box::new(localization))));
        self
    }

    /// Create a new application with the given name.
    pub fn from_name(name: impl Into<Box<str>>) -> Self {
        let (sender, receiver) = mpsc::channel();

        Application {
            request_sender: sender,
            name: name.into(),
            shell: Shell::new(receiver),
            theme: Rc::new(crate::widgets::themes::theme_orbtk::theme_default()),
            localization: None,
        }
    }

    /// Creates a new window and add it to the application.
    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let (adapter, settings, receiver) = create_window(
            self.name.clone(),
            &self.theme,
            self.request_sender.clone(),
            create_fn,
            self.localization.clone(),
        );

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
