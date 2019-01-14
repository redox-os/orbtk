use super::{Brush, Thickness};

pub trait Bordered {
    fn border_thickness(&self) -> &Thickness;
    fn set_border_thickness(&mut self, thickness: Thickness);
    fn border_brush(&self) -> &Brush;
    fn set_border_brush(&mut self, brush: Brush);
    fn border_radius(&self) -> f64;
    fn set_border_radius(&mut self, radius: f64);
    fn border(&self) -> &Border;
    fn set_border(&mut self, border: Border);
}


#[derive(Default)]
pub struct BorderBuilder {
    brush: Brush,
    thickness: Thickness,
    radius: f64,
}

impl BorderBuilder {
    pub fn new() -> BorderBuilder {
        BorderBuilder::default()
    }

    pub fn with_brush(mut self, brush: Brush) -> Self {
        self.brush = brush;
        self
    }

    pub fn with_thickness(mut self, thickness: Thickness) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn build(self) ->  Border {
        Border {
            brush: self.brush,
            thickness: self.thickness,
            radius: self.radius,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Border {
    pub brush: Brush,
    pub thickness: Thickness,
    pub radius: f64
}

impl Border {
    pub fn new(brush: Brush, thickness: Thickness, radius: f64) -> Self {
        Border {
            brush,
            thickness,
            radius,
        }
    }
}

