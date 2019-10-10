use crate::{prelude::*, render};

property!(
    /// `Image` describes an image property of a widget.
    #[derive(Default)]
    Image(render::Image) : &str,
    String
);

impl Image {
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    pub fn height(&self) -> f64 {
        self.0.height()
    }
}