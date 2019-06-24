use crate::{prelude::*, render::Image as Img};

property!(
    /// `Image` describes an image property of a widget.
    Image(Img)
);

impl Image {
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    pub fn height(&self) -> f64 {
        self.0.height()
    }
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::from(Img::new(s))
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::from(Img::new(s))
    }
}
