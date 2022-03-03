use std::{collections::HashMap, sync::mpsc};

use super::{Shell, Window};
use crate::{
    render::RenderContext2D, utils::Rectangle, window_adapter::WindowAdapter, WindowRequest,
    WindowSettings,
};

/// The `WindowBuilder` is used to construct an os independent window
/// shell that will communicate with the supported render backends.
pub struct WindowBuilder<'a, A: 'static>
where
    A: WindowAdapter,
{
    adapter: A,
    always_on_top: bool,
    borderless: bool,
    bounds: Rectangle,
    fonts: HashMap<String, &'static [u8]>,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    resizeable: bool,
    shell: &'a mut Shell<A>,
    title: String,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates the window builder from a settings object.
    pub fn from_settings(settings: WindowSettings, shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            adapter,
            always_on_top: settings.always_on_top,
            borderless: settings.borderless,
            bounds: Rectangle::new(settings.position, (settings.size.0, settings.size.1)),
            fonts: settings.fonts,
            request_receiver: None,
            resizeable: settings.resizeable,
            shell,
            title: settings.title,
        }
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell. The shell will be linked to the application `Shell`.
    pub fn build(self) {
        let mut render_context = RenderContext2D::new(self.bounds.width(), self.bounds.height());

        let mut flags = vec![];

        if self.resizeable {
            flags.push(orbclient::WindowFlag::Resizable);
        }

        if self.borderless {
            flags.push(orbclient::WindowFlag::Borderless);
        }

        if self.always_on_top {
            flags.push(orbclient::WindowFlag::Front);
        }

        let window = orbclient::Window::new_flags(
            self.bounds.x() as i32,
            self.bounds.y() as i32,
            self.bounds.width() as u32,
            self.bounds.height() as u32,
            self.title.as_str(),
            &flags,
        )
        .expect("WindowBuilder: Could not create an orblient window.");

        for (family, font) in self.fonts {
            render_context.register_font(&family, font);
        }

        self.shell.window_shells.push(Window::new(
            self.adapter,
            render_context,
            self.request_receiver,
            window,
        ));
    }

    /// Registers a new font via a string that will identify the font family.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            adapter,
            always_on_top: false,
            borderless: false,
            bounds: Rectangle::new((0.0, 0.0), (100.0, 75.0)),
            fonts: HashMap::new(),
            request_receiver: None,
            resizeable: false,
            shell,
            title: String::default(),
        }
    }

    /// Mark window as resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Register a window request receiver to communicate with the
    /// window shell via interprocess communication.
    pub fn request_receiver(mut self, request_receiver: mpsc::Receiver<WindowRequest>) -> Self {
        self.request_receiver = Some(request_receiver);
        self
    }

    /// Sets the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}
