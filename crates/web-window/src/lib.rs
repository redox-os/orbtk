
use stdweb::{
    self, _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{
        self, document, event,
        html_element::{CanvasElement, ImageElement},
        window, CanvasRenderingContext2d, FillRule,
    },
};

/// Used to initializes the web engine.
///
/// This method must be called first!
pub fn initialize() {
    stdweb::initialize();
}

/// Used to build a new web window.
#[derive(Default, Debug)]
pub struct WebWindowBuilder {
    title: String,
    size: (f64, f64),
}

impl WebWindowBuilder {
    /// Creates a new web window builder with default values.
    pub fn new() -> Self {
        WebWindowBuilder::default()
    }

    /// Used to set the `title` of the window.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Used to set the size of the window.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.size.0 = width;
        self.size.1 = height;
        self
    }

    pub fn build(self) -> WebWindow {
        document().set_title(&self.title[..]);

        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(self.size.0 as u32);
        canvas.set_height(self.size.1 as u32);

        stdweb::event_loop();

        WebWindow {
            title: self.title,
            size: self.size,
            canvas,
        }
    }
}


/// Web implementation of a window. Uses internal a canvas as window.
pub struct WebWindow {
    title: String,
    canvas: CanvasElement,
    size: (f64, f64),
}