use std::fmt;

use crate::utils::*;

#[derive(Clone, Default)]
pub struct RenderTarget {
    width: u32,
    height: u32,
    pub data: Vec<u32>,
}

impl fmt::Debug for RenderTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RenderTarget ( width: {}, height: {})",
            self.width, self.height
        )
    }
}

impl std::cmp::PartialEq for RenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl RenderTarget {
    /// Creates a new image with the given width and height.
    pub fn new(width: u32, height: u32) -> Self {
        RenderTarget {
            width,
            height,
            data: vec![Color::rgba(0, 0, 0, 0).data; width as usize * height as usize],
        }
    }

    /// Draws a u32 slice into the image.
    pub fn draw(&mut self, data: &[u32]) {
        self.data.clone_from_slice(data);
    }

    /// Create a new image from a boxed slice of colors
    pub fn from_data(width: u32, height: u32, data: Vec<u32>) -> Result<Self, String> {
        Ok(RenderTarget {
            width,
            height,
            data,
        })
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        self.width as f64
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.height as f64
    }

    pub fn data(&self) -> &[u32] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }
}
