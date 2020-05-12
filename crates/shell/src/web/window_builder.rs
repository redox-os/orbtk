use std::{cell::RefCell, char, collections::HashMap, rc::Rc, sync::mpsc, time::Duration};

use super::{Shell, Window};
use crate::{
    event::{ButtonState, Key, KeyEvent},
    render::RenderContext2D,
    utils::Rectangle,
    WindowRequest,
    window_adapter::WindowAdapter,
};

/// The `WindowBuilder` is used to construct a window shell for the web backend.
pub struct WindowBuilder<'a, A>
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
    request_receiver: Option<mpsc::Receiver<WindowRequest>>
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
            bounds: Rectangle::new(0.0, 0.0, 100.0, 75.0),
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
      
        self.shell.window_shells.push(Window::new(
            self.adapter,
            // render_context,
            self.request_receiver,
            true,
            true,
            false
        ));
    }
}
