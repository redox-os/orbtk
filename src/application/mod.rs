use std::sync::Arc;

use {Rect, Theme};
pub use self::tree_manager::*;

pub use self::window::*;

use self::default_backend::default_backend;

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
            backend: default_backend(theme),
        }
    }

    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[path="default_backend/orbital.rs"]
mod default_backend;

#[cfg(target_arch = "wasm32")]
#[path="default_backend/wasm.rs"]
mod default_backend;