use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use orbclient::Renderer;
use orbgl_api::Color;
use orbgl_api::Image as OrbImage;

use crate::prelude::*;

#[derive(Clone)]
pub struct InnerImage(pub OrbImage);

impl Default for InnerImage {
    #[cfg(not(target_arch = "wasm32"))]
    fn default() -> Self {
        InnerImage(OrbImage::new(0, 0))
    }

    #[cfg(target_arch = "wasm32")]
    fn default() -> Self {
        InnerImage(OrbImage::new())
    }
}

impl fmt::Debug for InnerImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InnerImage(orbimage::Image)")
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
pub trait ImageExt {
    /// Gets the width.
    fn width(&self) -> u32;

    /// Gets the height.
    fn height(&self) -> u32;

    #[cfg(not(target_arch = "wasm32"))]
    /// Gets the color data.
    fn data(&self) -> &[Color];
}

impl ImageExt for InnerImage {
    fn width(&self) -> u32 {
        self.0.width()
    }

    fn height(&self) -> u32 {
        self.0.height()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn data(&self) -> &[Color] {
        self.0.data()
    }
}

impl ImageExt for Image {
    fn width(&self) -> u32 {
        self.0.width()
    }

    fn height(&self) -> u32 {
        self.0.height()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn data(&self) -> &[Color] {
        self.0.data()
    }
}

// --- Conversions ---

impl From<&str> for Image {
    #[cfg(not(target_arch = "wasm32"))]
    fn from(s: &str) -> Image {
        Image::from(InnerImage::from(OrbImage::from_path(s).unwrap()))
    }

    #[cfg(target_arch = "wasm32")]
    fn from(s: &str) -> Image {
        Image::from(InnerImage::from(OrbImage::new()))
    }
}

impl From<String> for Image {
    #[cfg(not(target_arch = "wasm32"))]
    fn from(s: String) -> Image {
        Image::from(InnerImage::from(OrbImage::from_path(s).unwrap()))
    }

    #[cfg(target_arch = "wasm32")]
    fn from(s: String) -> Image {
        Image::from(InnerImage::from(OrbImage::new()))
    }
}
