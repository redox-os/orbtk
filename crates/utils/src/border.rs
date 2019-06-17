use super::{Brush, Thickness};

/// Used to build a border, specifying additional details.
#[derive(Default)]
pub struct BorderBuilder {
    brush: Brush,
    thickness: Thickness,
    radius: f64,
}

impl BorderBuilder {
    /// Creates a new border builder with default values.
    pub fn new() -> BorderBuilder {
        BorderBuilder::default()
    }

    /// Inserts a border brush.
    pub fn brush<B: Into<Brush>>(mut self, brush: B) -> Self {
        self.brush = brush.into();
        self
    }

    /// Inserts a border thickness.
    pub fn thickness(mut self, thickness: Thickness) -> Self {
        self.thickness = thickness;
        self
    }

    /// Inserts a border radius.
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Builds the border.
    pub fn build(self) -> Border {
        Border {
            brush: self.brush,
            thickness: self.thickness,
            radius: self.radius,
        }
    }
}

/// Describes a border of a shape with border `brush`, `thickness` and `radius`.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Border {
    brush: Brush,
    thickness: Thickness,
    radius: f64,
}

impl Border {
    /// Creates a new `BorderBuilder` with default values.
    pub fn create() -> BorderBuilder {
        BorderBuilder::new()
    }

    /// Gets the border brush.
    pub fn brush(&self) -> &Brush {
        &self.brush
    }

    /// Sets the border brush.
    pub fn set_brush<B: Into<Brush>>(&mut self, brush: B) {
        self.brush = brush.into();
    }

    /// Gets the thickness.
    pub fn thickness(&self) -> Thickness {
        self.thickness
    }

    /// Sets the thickness.
    pub fn set_thickness(&mut self, thickness: Thickness) {
        self.thickness = thickness;
    }

    /// Gets the radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Sets the radius.
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius
    }
}

/// Contains a set of getters and setters to read and write to a border.
pub trait Bordered {
    /// Gets the thickness.
    fn border_thickness(&self) -> Thickness;

    /// Sets the border thickness.
    fn set_border_thickness(&mut self, thickness: Thickness);

    /// Gets the border brush.
    fn border_brush(&self) -> &Brush;

    /// Sets the border brush.
    fn set_border_brush(&mut self, brush: Brush);

    /// Gets the border radius.
    fn border_radius(&self) -> f64;

    /// Sets the border radius.
    fn set_border_radius(&mut self, radius: f64);

    /// Gets the complete border.
    fn border(&self) -> &Border;

    /// Sets the complete border.
    fn set_border(&mut self, border: Border);
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_brush() {
        let brush = Brush::from("#000000");

        let builder = BorderBuilder::new();
        let border = builder.brush(brush).build();

        let test_brush = Brush::from("#000000");
        assert_eq!(border.brush(), &test_brush);
    }

    #[test]
    fn test_thickness() {
        let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

        let builder = BorderBuilder::new();
        let border = builder.thickness(thickness).build();
        assert_eq!(border.thickness(), thickness);
    }

    #[test]
    fn test_radius() {
        let radius = 0.0;

        let builder = BorderBuilder::new();
        let border = builder.radius(radius).build();
        assert_eq!(border.radius(), radius);
    }

    #[test]
    fn test_set_brush() {
        let brush = Brush::from("#000000");

        let mut border = Border::default();
        border.set_brush(brush);

        let test_brush = Brush::from("#000000");
        assert_eq!(border.brush(), &test_brush);
    }

    #[test]
    fn test_set_thickness() {
        let thickness = Thickness::new(0.0, 0.0, 0.0, 0.0);

        let mut border = Border::default();
        border.set_thickness(thickness);
        assert_eq!(border.thickness(), thickness);
    }

    #[test]
    fn test_set_radius() {
        let radius = 0.0;

        let mut border = Border::default();
        border.set_radius(radius);
        assert_eq!(border.radius(), radius);
    }
}
