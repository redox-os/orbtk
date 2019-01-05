//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;
use std::thread;

use orbrender;
use orbrender::backend::Runner;
use orbrender::traits::Window;

use {Bounds, Theme};

pub use self::global::*;
pub use self::tree::*;
pub use self::window::*;

mod global;
mod tree;
mod window;

pub fn initialize() {
    orbrender::initialize();
}

#[derive(Default)]
/// The `Application` represents the entry point of an OrbTk based application.
pub struct Application {
    main_window_runner: Option<Runner>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn main_window(&mut self, window: Box<Window>) -> WindowSupplier {
        WindowSupplier {
            application: self,
            window,
            theme: Theme::new(),
            root: None,
            debug_flag: false,
        }
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
       if let Some(runner) = self.main_window_runner {
           runner.run();
       }
    }
}
