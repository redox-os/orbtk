/// Base image trait.
pub trait Image {
    /// Gets the width of the image.
    fn width(&self) -> f64;

    /// Gets the height of the image.
    fn height(&self) -> f64;
}
