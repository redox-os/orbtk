//! This module contains a platform specific implementation of the window shell.

use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc, sync::Arc};

use stdweb::{
    _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{
        self, document, event,
        html_element::{CanvasElement, ImageElement},
        window, CanvasRenderingContext2d, FillRule,
    },
};

use orbgl_api::{Canvas, Font};
use orbgl_web::prelude::*;

use orbtk_utils::{Point, Rect};

use crate::{obsolete, prelude::*};

pub fn initialize() {
    stdweb::initialize();
}

/// Concrete implementation of the window shell.
pub struct WindowShell<A> where A: WindowAdapter {
    pub inner: WebRenderer,
    pub canvas: Canvas,
    adapter: A
}

impl<A> WindowShell<A> where A: WindowAdapter {
    /// Creates a new window shell with an adapter.
    pub fn new(inner: WebRenderer, canvas: Canvas, adapter: A) -> WindowShell<A> {
        WindowShell {
            inner,
            canvas,
            adapter,
        }
    }

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    fn drain_events(&mut self) {
    }
}

// impl<A> Drop for WindowShell<A> where A: WindowAdapter {
//     fn drop(&mut self) {
//         self.inner.sync();
//     }
// }

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A> where A: WindowAdapter + 'static {
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub updater: Box<Updater>,
}

impl<A> ShellRunner<A> where A: WindowAdapter {
    pub fn run(mut self) {

        window().request_animation_frame(move |_| {
            self.updater.update();
            self.update.set(false);
        });
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A> where A: WindowAdapter {
    title: String,

    resizeable: bool,

    bounds: Rect,

    adapter: A,
}

impl<A> WindowBuilder<A> where A: WindowAdapter {
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        WindowBuilder {
            adapter,
            title: String::default(),
            resizeable: false,
            bounds: Rect::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rect>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(self) -> WindowShell<A> {

        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        
        canvas.set_width(self.bounds.width as u32);
        canvas.set_height(self.bounds.height as u32);

        document().body().unwrap().append_child(&canvas);

        let surface = WebSurface::new(self.bounds.width as u32, self.bounds.height as u32, canvas.get_context().unwrap());
        let render_engine = WebRenderEngine::new(surface);
        let mut canvas = Canvas::new(render_engine.clone());

        document().set_title(&self.title[..]);

        stdweb::event_loop();

        WindowShell::new(
            WebRenderer {},
            canvas,
            self.adapter,
        )
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---

pub struct WebFontMeasure;

impl FontMeasure for WebFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        (0, 0)
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<WebFontMeasure> = { Arc::new(WebFontMeasure) };
}

pub struct WebRenderer {

}

impl obsolete::Renderer for WebRenderer {
    fn render_text(
        &mut self,
        _text: &str,
        _bounds: &Rect,
        _parent_bounds: &Rect,
        _global_position: &Point,
        _font_size: u32,
        _color: Color,
        _font: &Font,
    ) {
       
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---