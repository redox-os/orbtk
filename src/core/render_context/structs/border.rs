use super::{Brush, Thickness};

pub trait Bordered {
    fn get_border_thickness(&self) -> &Thickness;
    fn set_border_thickness(&mut self, thickness: Thickness);
    fn get_border_brush(&self) -> &Brush;
    fn set_border_brush(&mut self, brush: Brush);
    fn get_border(&self) -> &Border;
    fn set_border(&mut self, border: Border);
}


#[derive(Default)]
pub struct BorderBuilder {
    brush: Brush,
    thickness: Thickness,
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

    pub fn build(self) ->  Border {
        Border {
            brush: self.brush,
            thickness: self.thickness,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Border {
    pub brush: Brush,
    pub thickness: Thickness,
}

impl Border {
    pub fn new(brush: Brush, thickness: Thickness) -> Self {
        Border {
            brush,
            thickness,
        }
    }
}

