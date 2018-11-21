use {Rect, Theme};

pub use self::global::*;
pub use self::window::*;

mod global;
mod window;

#[derive(Default)]
pub struct Application {
    windows: Vec<Window>,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_window(&mut self) -> WindowBuilder {
        WindowBuilder {
            application: self,
            bounds: Rect::default(),
            title: String::from(""),
            theme: Theme::new(),
            root: None,
            debug_flag: false,
        }
    }

    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}
