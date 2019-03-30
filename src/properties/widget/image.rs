use std::fmt;

use orbclient::Color;
use orbclient::Renderer;
use orbimage::Image as OrbImage;

#[derive(Clone)]
pub struct InnerImage(pub OrbImage);

impl Default for InnerImage {
    fn default() -> Self {
        InnerImage(OrbImage::new(0, 0))
    }
}

impl fmt::Debug for InnerImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InnerImage(orbimage::Image)")
    }
}

impl PartialEq for InnerImage {
    // todo: impl
    fn eq(&self, _other: &InnerImage) -> bool {
        false
    }
}

impl From<OrbImage> for InnerImage {
    fn from(image: OrbImage) -> InnerImage {
        InnerImage(image)
    }
}

property!(
    /// `Image` describes an image property of a widget.
    Image(InnerImage)
);

// --- Trait implementations ---

/// Provides additional methods for image objects.
pub trait ImageExtension {
    /// Gets the width.
    fn width(&self) -> u32;

    /// Gets the height.
    fn height(&self) -> u32;

    /// Gets the color data.
    fn data(&self) -> &[Color];
}

impl ImageExtension for InnerImage {
    fn width(&self) -> u32 {
        self.0.width()
    }

    fn height(&self) -> u32 {
        self.0.height()
    }

    fn data(&self) -> &[Color] {
        self.0.data()
    }
}

impl ImageExtension for Image {
    fn width(&self) -> u32 {
        self.0.width()
    }

    fn height(&self) -> u32 {
        self.0.height()
    }

    fn data(&self) -> &[Color] {
        self.0.data()
    }
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::from(InnerImage::from(OrbImage::from_path(s).unwrap()))
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::from(InnerImage::from(OrbImage::from_path(s).unwrap()))
    }
}
