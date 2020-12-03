use std::{fmt, path::Path};

use crate::RenderTarget;

#[derive(Clone, Default)]
pub struct Image {
    render_target: RenderTarget,
    source: String,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image ( source: {})", self.source)
    }
}

impl std::cmp::PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Image {
    /// Creates a new image with the given width and height.
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            render_target: RenderTarget::new(width, height),
            source: String::default(),
        }
    }

    /// Draws a u32 slice into the image.
    pub fn draw(&mut self, data: &[u32]) {
        self.render_target.data.clone_from_slice(data);
    }

    /// Create a new image from a boxed slice of colors
    pub fn from_data(width: u32, height: u32, data: Vec<u32>) -> Result<Self, String> {
        Ok(Image {
            render_target: RenderTarget::from_data(width, height, data).unwrap(),
            source: String::new(),
        })
    }

    /// Creates a new image from an `RgbaImage`.
    pub fn from_rgba_image(image: image::RgbaImage) -> Result<Self, String> {
        let data: Vec<u32> = image
            .pixels()
            .map(|p| {
                ((p[3] as u32) << 24) | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
            })
            .collect();
        Self::from_data(image.width(), image.height(), data)
    }

    /// Load an image from file path. Supports BMP and PNG
    pub fn from_path<P: AsRef<Path> + std::fmt::Debug + Clone>(path: P) -> Result<Self, String> {
        let img = image::open(path.clone());

        if let Ok(img) = img {
            return Self::from_rgba_image(img.to_rgba8());
        }

        Err(format!("Could not load image width path: {:?}", path))
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        self.render_target.width() as f64
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.render_target.height() as f64
    }

    pub fn data(&self) -> &[u32] {
        &self.render_target.data
    }

    pub fn data_mut(&mut self) -> &mut [u32] {
        &mut self.render_target.data
    }
}

impl From<(u32, u32, Vec<u32>)> for Image {
    fn from(image: (u32, u32, Vec<u32>)) -> Self {
        Image::from_data(image.0, image.1, image.2).unwrap()
    }
}

pub fn os_path(path: String) -> String {
    if cfg!(windows) {
        path.replace("/", "\\")
    } else {
        path.replace("\\", "/")
    }
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::from_path(os_path(s.to_string())).unwrap()
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::from_path(os_path(s)).unwrap()
    }
}

// --- Conversions ---
