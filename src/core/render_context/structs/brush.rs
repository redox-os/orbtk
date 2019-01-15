
/// Describes a position on a coloerfull gradient.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub position: f64,
    pub color: String,
}

/// A `Brush`describes how a shape is filled or stroked.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Paints an area with a solid color.
    SolidColor(String),

    /// Paints an area with a linear gradient.
    Gradient(Vec<GradientStop>),
}

impl Default for Brush {
    fn default() -> Self {
        Brush::SolidColor("#000000".to_string())
    }
}

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Brush::SolidColor(s.to_string())
    }
}

impl From<String> for Brush {
    fn from(s: String) -> Brush {
        Brush::SolidColor(s)
    }
}

impl From<Vec<GradientStop>> for Brush {
    fn from(gradient: Vec<GradientStop>) -> Brush {
        Brush::Gradient(gradient)
    }
}