// This private sub module contains a list of shapes.

pub use self::image_element::{ImageElement, ImageElementBuilder};
pub use self::rectangle::{Rectangle, RectangleBuilder};
pub use self::path_segment::PathSegment;

mod image_element;
mod rectangle;
mod path_segment;

/// Provides the base for render shapes like `Rectangle`, `ImageElement` and `Text`.
pub trait Shape {
    /// Gets the path of render `PathSegements`.
    fn path(&mut self) -> &mut [PathSegment];

    /// Builds the inner path of render `PathSegments`.
    fn build_path(&mut self);
}
