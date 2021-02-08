use orbclient::Renderer;

use morph_ui_api::api::BuildContext;
use morph_ui_shell::{Shell, ShellBuilder};

use crate::Error;

/// Defines a top-level window on the screen.
pub struct Window {
    inner: orbclient::Window,
    shell: Shell,
}

impl Window {
    /// Creates a new window builder that is used to configure the window.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use morph_ui_orbclient::Window;
    /// // use morph_ui::orbclient::Window;
    ///
    /// let window = Window::create().position(0, 0).size(640, 480).title("Window").build();
    /// ```
    pub fn create() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// Drain events and propagate the events to the shell.
    ///
    /// If it return `false` the window should be closed.
    pub fn drain_events(&mut self) -> bool {
        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Quit(_) => {
                    self.shell.quit();
                    return false;
                }
                orbclient::EventOption::Key(e) => {
                    self.shell.key(e.scancode, e.character, e.pressed)
                }
                orbclient::EventOption::TextInput(e) => {
                    self.shell.text_input(e.character.to_string())
                }
                orbclient::EventOption::Mouse(e) => self.shell.mouse(e.x as f64, e.y as f64),
                orbclient::EventOption::MouseRelative(_) => println!("no yet implemented"),
                orbclient::EventOption::Button(e) => {
                    self.shell.mouse_button(e.left, e.middle, e.right)
                }
                orbclient::EventOption::Scroll(e) => self.shell.scroll(e.x as f64, e.y as f64),
                orbclient::EventOption::Focus(e) => self.shell.active(e.focused),
                orbclient::EventOption::Move(e) => self.shell.moved(e.x as f64, e.y as f64),
                orbclient::EventOption::Resize(e) => self.shell.resize(e.width, e.height),
                orbclient::EventOption::Screen(_) => println!("no yet implemented"),
                orbclient::EventOption::Clipboard(_) => println!("no yet implemented"),
                orbclient::EventOption::ClipboardUpdate(_) => println!("no yet implemented"),
                orbclient::EventOption::Drop(_) => println!("no yet implemented"),
                orbclient::EventOption::Hover(_) => println!("no yet implemented"),
                orbclient::EventOption::Unknown(_) => println!("no yet implemented"),
                orbclient::EventOption::None => println!("no yet implemented"),
            }
        }

        // todo move to correct code part.
        self.shell.update_and_draw();

        let bytes = self.shell.frame_buffer_mut();
        let color_data = unsafe {
            std::slice::from_raw_parts_mut(
                bytes.as_mut_ptr() as *mut orbclient::Color,
                bytes.len() / std::mem::size_of::<orbclient::Color>(),
            )
        };

        if color_data.len() == self.inner.data().len() {
            self.inner.data_mut().clone_from_slice(color_data);
        }

        self.inner.sync();

        true
    }

    /// Runs the inner logic of the window.
    pub fn run(&mut self) -> bool {
        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Quit(_) => return false,
                _ => {}
            }
        }

        true
    }
}

/// Window builder is used to configure a window and create it.
#[derive(Debug)]
pub struct WindowBuilder {
    shell_builder: ShellBuilder,
    position: (i32, i32),
    size: (u32, u32),
    title: String,
    resizeable: bool,
    borderless: bool,
    centered: bool,
}

impl WindowBuilder {
    /// Creates a new window builder.
    fn new() -> Self {
        WindowBuilder {
            shell_builder: Shell::create(),
            position: (0, 0),
            size: (0, 0),
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

    /// Builder method to define the ui of the window.
    ///
    /// An ui can only be set once. If the method is multiple called, the last set ui will be used.
    pub fn ui<F: Fn(&mut BuildContext)>(mut self, builder: F) -> Self {
        self.shell_builder = self
            .shell_builder
            .size(self.size.0, self.size.1)
            .ui(builder);
        self
    }

    /// Creates a new window with the given builder settings.
    pub fn build(self) -> Result<Window, Error> {
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
            return Ok(Window {
                inner,
                shell: self.shell_builder.build(),
            });
        }

        return Err(Error::CannotCreateWindow);
    }
}
