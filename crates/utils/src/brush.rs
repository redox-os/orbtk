use crate::prelude::*;

/// A `Brush`describes how a shape is filled or stroked.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Paints an area with a solid color.
    SolidColor(Color),

    /// Paints an area with a gradient.
    Gradient(Gradient),

    Stacked(Vec<Brush>),
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

impl From<Brush> for Gradient {
    fn from(b: Brush) -> Gradient {
        match b {
            Brush::Gradient(g) => g,
            _ => Gradient::default(),
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

impl From<Gradient> for Brush {
    fn from(g: Gradient) -> Brush {
        Brush::Gradient(g)
    }
}

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Property::from(s).brush().unwrap_or_default()
    }
}

impl From<String> for Brush {
    fn from(s: String) -> Brush {
        Self::from(&s[..])
    }
}

impl From<Value> for Brush {
    fn from(v: Value) -> Self {
        let value = v.get::<String>();
        Brush::from(value)
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
