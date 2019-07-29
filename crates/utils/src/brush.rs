use crate::prelude::*;

/// Describes a position on a colorful gradient.
#[derive(Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub position: f64,
    pub color: Color,
}

/// A `Brush`describes how a shape is filled or stroked.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Paints an area with a solid color.
    SolidColor(Color),

    /// Paints an area with a linear gradient.
    Gradient(Vec<GradientStop>),
}

impl From<Brush> for Color {
    fn from(b: Brush) -> Color {
        match b {
            Brush::SolidColor(color) => color.clone(),
            _ => Color::rgb(0, 0, 0),
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Brush::SolidColor(Color::rgb(0, 0, 0))
    }
}

impl From<Color> for Brush {
    fn from(c: Color) -> Brush {
        Brush::SolidColor(c)
    }
}

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Brush::SolidColor(Color::from(s))
    }
}

impl From<String> for Brush {
    fn from(s: String) -> Brush {
        Brush::SolidColor(Color::from(s))
    }
}

impl From<Vec<GradientStop>> for Brush {
    fn from(gradient: Vec<GradientStop>) -> Brush {
        Brush::Gradient(gradient)
    }
}

#[cfg(test)]
mod tests {
    //  use crate::prelude::*;
    // todo: tbd after brush struct is finished
}
