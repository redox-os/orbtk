use std::{collections::HashMap, sync::mpsc};

use super::{Shell, Window};
use crate::{
    render::RenderContext2D, utils::Rectangle, window_adapter::WindowAdapter, WindowRequest,
    WindowSettings,
};

/// The `WindowBuilder` is used to construct a window shell for the minifb backend.
pub struct WindowBuilder<'a, A: 'static>
where
    A: WindowAdapter,
{
    shell: &'a mut Shell<A>,
    adapter: A,
    title: String,
    resizeable: bool,
    always_on_top: bool,
    borderless: bool,
    fonts: HashMap<String, &'static [u8]>,
    bounds: Rectangle,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: String::default(),
            resizeable: false,
            always_on_top: false,
            borderless: false,
            fonts: HashMap::new(),
            bounds: Rectangle::new((0.0, 0.0), (100.0, 75.0)),
            request_receiver: None,
        }
    }

    /// Creates the window builder from a settings object.
    pub fn from_settings(settings: WindowSettings, shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: settings.title,
            resizeable: settings.resizeable,
            always_on_top: settings.always_on_top,
            borderless: settings.borderless,
            fonts: settings.fonts,
            bounds: Rectangle::new(settings.position, (settings.size.0, settings.size.1)),
            request_receiver: None,
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Registers a new font with family key.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    /// Register a window request receiver to communicate with the window shell from outside.
    pub fn request_receiver(mut self, request_receiver: mpsc::Receiver<WindowRequest>) -> Self {
        self.request_receiver = Some(request_receiver);
        self
    }

    /// Builds the window shell and add it to the application `Shell`.
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
        .expect("WindowBuilder: Could no create an orblient window.");

        for (family, font) in self.fonts {
            render_context.register_font(&family, font);
        }

        self.shell.window_shells.push(Window::new(
            window,
            self.adapter,
            render_context,
            self.request_receiver,
        ));
    }
}
