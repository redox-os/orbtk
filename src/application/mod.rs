use std::cell::RefCell;
use std::sync::Arc;

use {OrbitalBackend, Rect, Theme};

pub use self::tree_manager::*;
pub use self::window::*;

mod tree_manager;
mod window;

pub struct Application {
    // list of windows
    // theme
    theme: Arc<Theme>,
    windows: Vec<Window>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            theme: Arc::new(Theme::new()),
            windows: vec![],
        }
    }

    pub fn create_window(&mut self) -> WindowBuilder {
        let theme = self.theme.clone();
        WindowBuilder {
            application: self,
            bounds: Rect::default(),
            title: String::from(""),
            theme: theme.clone(),
            root: None,
            backend: Arc::new(RefCell::new(OrbitalBackend::new(theme))),
        }
    }

    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}
