use std::{cell::RefCell, char, collections::HashMap, rc::Rc, sync::mpsc, time::Duration};

use glutin::{
    dpi::{LogicalSize, PhysicalSize},
    window, ContextBuilder, GlProfile, GlRequest,
};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::gpu::{
    options::{DestFramebuffer, RendererOptions},
    renderer::Renderer,
};
use pathfinder_resources::embedded::EmbeddedResourceLoader;

use super::{Shell, Window};

use crate::{
    event::{ButtonState, Key, KeyEvent},
    render::RenderContext2D,
    utils::Rectangle,
    window_adapter::WindowAdapter,
    WindowRequest,
};

/// The `WindowBuilder` is used to construct a window shell for the minifb backend.
pub struct WindowBuilder<'a, A: 'static>
where
    A: WindowAdapter,
{
    window_builder: window::WindowBuilder,
    shell: &'a mut Shell<A>,
    adapter: A,
    fonts: HashMap<String, &'static [u8]>,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    bounds: Rectangle,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            window_builder: window::WindowBuilder::new(),
            shell,
            adapter,
            fonts: HashMap::new(),
            request_receiver: None,
            bounds: Rectangle::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.window_builder = self.window_builder.with_title(title);
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.window_builder = self.window_builder.with_decorations(!borderless);
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.window_builder = self.window_builder.with_resizable(resizeable);
        self
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.window_builder = self.window_builder.with_always_on_top(always_on_top);
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        let window_size = (self.bounds.width(), self.bounds.height());
        let physical_size = PhysicalSize::new(window_size.0, window_size.1);

        self.window_builder = self.window_builder.with_inner_size(physical_size);
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
        // Create an OpenGL 3.x context for Pathfinder to use.
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(self.window_builder, self.shell.event_loop())
            .unwrap();

        // Load OpenGL, and make the context current.
        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        let window_size = (self.bounds.width(), self.bounds.height());

        let logical_size = PhysicalSize::new(window_size.0, window_size.1);

        // Create a Pathfinder renderer.
        let mut renderer = Renderer::new(
            GLDevice::new(GLVersion::GL3, 0),
            &EmbeddedResourceLoader::new(),
            DestFramebuffer::full_window(vec2i(window_size.0 as i32, window_size.1 as i32)),
            RendererOptions {
                background_color: Some(ColorF::white()),
                ..RendererOptions::default()
            },
        );

        let render_context = RenderContext2D::new_ex(window_size, renderer);

        self.shell.window_shells.push(Window::new(
            gl_context,
            self.adapter,
            render_context,
            self.request_receiver,
            true,
            true,
            false,
        ))
    }
}
