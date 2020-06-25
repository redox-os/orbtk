use crate::prelude::*;

/// Describes a position on a colorful gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LinearGradientStop {
    pub position: f64,
    pub color: Color,
}

/// A `Brush`describes how a shape is filled or stroked.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Paints an area with a solid color.
    SolidColor(Color),

    /// Paints an area with a linear gradient.
    LinearGradient {
        start: Point,
        end: Point,
        stops: Vec<LinearGradientStop>,
    },
}

impl Brush {
    pub fn is_transparent(&self) -> bool {
        match self {
            Brush::SolidColor(color) => color.a() == 0,
            _ => false,
        }
    }
}

impl From<Brush> for Color {
    fn from(b: Brush) -> Color {
        match b {
            Brush::SolidColor(color) => color,
            _ => Color::rgb(0, 0, 0),
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Brush::SolidColor(Color::rgba(0, 0, 0, 0))
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

impl From<ron::Value> for Brush {
    fn from(v: ron::Value) -> Self {
        if let Ok(value) = v.into_rust::<String>() {
            return Brush::from(value);
        }

        Brush::default()
    } 
}

// impl From<Vec<LinearGradientStop>> for Brush {
//     fn from(gradient: Vec<LinearGradientStop>) -> Brush {
//         Brush::LinearGradient(gradient)
//     }
// }

#[cfg(test)]
mod tests {
    //  use crate::prelude::*;
    // todo: tbd after brush struct is finished
}
