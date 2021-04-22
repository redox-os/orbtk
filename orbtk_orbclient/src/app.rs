use crate::*;

/// Manges the initial setup of the application, its startup and the application loop.
pub struct App {
    windows: Vec<Box<dyn Runner>>,
}

impl App {
    /// Creates a new app, that can be configured using the builder pattern.
    pub fn new() -> Self {
        App { windows: vec![] }
    }

    /// Builder method that is used add a new window to the app.
    pub fn window<S>(mut self, builder: WindowBuilder<S>) -> Result<Self, Error>
    where
        S: 'static + Default + Clone + PartialEq,
    {
        self.windows.push(Box::new(builder.build()?));
        Ok(self)
    }

    // Starts and runs the application (not wasm32 target)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start(mut self) -> Result<(), Error> {
        loop {
            if !self.run()? {
                break;
            }
        }

        Ok(())
    }

    // Starts and runs the application (wasm32 target)
    #[cfg(target_arch = "wasm32")]
    pub fn start(mut self) -> Result<bool, Error> {
        orbclient::animation_loop(move || {
            if let Ok(run) = self.run() {
                return run;
            }

            return false;
        });

        Ok(true)
    }

    // Runs the loops of the windows until no window is left.
    fn run(&mut self) -> Result<bool, Error> {
        for i in 0..self.windows.len() {
            if i > self.windows.len() - 1 {
                break;
            }

            // removes the window from the list if run returns false
            if !self.windows[i].run()? {
                self.windows.remove(i);
            }
        }

        // if no window is left, close the application.
        if self.windows.is_empty() {
            return Ok(false);
        }

        Ok(true)
    }
}
