use crate::{prelude::*, render::Image as Img};

property!(
    /// `Image` describes an image property of a widget.
    Image(Img) : &str,
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
