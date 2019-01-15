use super::{Brush, Thickness};

/// Used to build a border, specifying additional details.
#[derive(Default)]
pub struct BorderBuilder {
    brush: Brush,
    thickness: Thickness,
    radius: f64,
}

impl BorderBuilder {
    /// Creates a new border bilder with default values.
    pub fn new() -> BorderBuilder {
        BorderBuilder::default()
    }

    /// Inserts a border brsuh.
    pub fn with_brush(mut self, brush: Brush) -> Self {
        self.brush = brush;
        self
    }

    /// Inserts a border thickness.
    pub fn with_thickness(mut self, thickness: Thickness) -> Self {
        self.thickness = thickness;
        self
    }

    /// Inserts a border radius.
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Builds the border.
    pub fn build(self) ->  Border {
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
    radius: f64
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
    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
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