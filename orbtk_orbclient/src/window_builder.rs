use atomic_refcell::*;

use crate::*;

use orbtk_core::*;
use orbtk_tiny_skia::*;

/// Window builder is used to configure a window and create it.
pub struct WindowBuilder<S>
where
    S: Default + Clone + PartialEq,
{
    position: (i32, i32),
    size: (u32, u32),
    title: String,
    resizeable: bool,
    borderless: bool,
    centered: bool,
    ui: Ui<S>,
}

impl<S> WindowBuilder<S>
where
    S: Default + Clone + PartialEq,
{
    /// Creates a new window builder.
    pub fn new(state: S) -> Self {
        Self {
            ui: Ui::new(state),
            position: (0, 0),
            size: (100, 100),
            title: String::default(),
            resizeable: false,
            borderless: false,
            centered: false,
        }
    }

    /// Builder method that is used to specify the window position on the screen with x and y.
    ///
    /// If centered is set to `true`, position will be ignored.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = (x, y);
        self
    }

    /// Builder method that is used to specify the window size with width and height.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    /// Builder method that is used to specify the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Builder method that is used to specify the window resizsablility.
    ///
    /// If resizeable is set to `true` the window can resized otherwise the size of the window
    /// will be fixed.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Builder method that is used to specify the window borderless.
    ///
    /// If borderless is set to `true` the window will be displayed without window borders, otherwise
    /// with borders.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Builder method that is used to place the window in the center of the screen.
    ///
    /// If set to `true` the window will be centered on the screen an position will be ignored.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Builder method that is used to define the view builder fn for the ui of the window.
    pub fn view<F>(mut self, view_builder: F) -> Self
    where
        F: Fn(&mut S) -> BuildContext + 'static,
    {
        self.ui.set_view(view_builder);
        self
    }

    /// Creates a new window with the given builder settings.
    pub fn build(self) -> Result<Window<S>, Error> {
        let mut flags = vec![];

        if self.resizeable {
            flags.push(orbclient::WindowFlag::Resizable);
        }

        if self.borderless {
            flags.push(orbclient::WindowFlag::Front);
        }

        // used to center the window on the screen if centered is set to true
        let (x, y) = {
            if self.centered {
                let screen_size =
                    orbclient::get_display_size().map_err(|_| Error::CannotReadScreenSize)?;

                (
                    (screen_size.0 as i32 - self.size.0 as i32) / 2,
                    (screen_size.1 as i32 - self.size.1 as i32) / 2,
                )
            } else {
                (self.position.0, self.position.1)
            }
        };

        // create the window
        if let Some(inner) = orbclient::Window::new_flags(
            x,
            y,
            self.size.0,
            self.size.1,
            self.title.as_str(),
            &flags,
        ) {
            return Ok(Window { inner, ui: self.ui });
        }

        Err(Error::CannotCreateWindow)
    }
}
