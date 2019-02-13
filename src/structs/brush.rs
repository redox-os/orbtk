use orbgl_api::Color;

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

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Brush::SolidColor(get_color(s))
    }
}

impl From<String> for Brush {
    fn from(s: String) -> Brush {
        Brush::SolidColor(get_color(&s))
    }
}

impl From<Vec<GradientStop>> for Brush {
    fn from(gradient: Vec<GradientStop>) -> Brush {
        Brush::Gradient(gradient)
    }
}

// Todo: this helper could me removed after orbgl_api is providing an own color struct
fn get_color(hex: &str) -> Color {
    let clean_hex = hex.trim_start_matches("#");
    match clean_hex.len() {
        6 | 8 => {
            let mut x = match u32::from_str_radix(&clean_hex, 16) {
                Ok(x) => x,
                Err(_) => 0,
            };

            if clean_hex.len() == 6 {
                x |= 0xFF_000_000;
            }

            Color { data: x }
        }
        _ => Color { data: 0 },
    }
}
