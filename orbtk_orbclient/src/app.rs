use crate::{Error, Window, WindowBuilder};

/// Manges the initial setup of the application, its startup and the application loop.
pub struct App {
    windows: Vec<Window>,
}

impl App {
    /// Creates a new app builder that is used to configure the app.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use morph_ui_orbclient::{App, Window};
    /// // use morph_ui::orbclient::{App, Window};
    ///
    /// App::create().window(
    ///    Window::create().position(0, 0).size(640, 480).title("Window")
    /// ).unwrap().start();
    /// ```
    pub fn create() -> AppBuilder {
        AppBuilder::new()
    }

    // start and runs the application
    fn run(mut self) {
        // todo: handle animation loop.
        loop {
            for i in 0..self.windows.len() {
                // removes the window from the list if run returns false
                if !self.windows[i].drain_events() {
                    self.windows.remove(i);
                }
            }

            // if no window is left, close the application.
            if self.windows.is_empty() {
                break;
            }
        }
    }
}

/// App builder is used to configure, create and start an app.
pub struct AppBuilder {
    windows: Vec<Window>,
}

impl AppBuilder {
    // Creates a new app builder.
    fn new() -> Self {
        AppBuilder { windows: vec![] }
    }

    /// Builder method that is used add a new window to the app.
    pub fn window(mut self, builder: WindowBuilder) -> Result<Self, Error> {
        self.windows.push(builder.build()?);
        Ok(self)
    }

    /// Starts the application.
    pub fn start(self) {
        App {
            windows: self.windows,
        }
        .run()
    }
}
