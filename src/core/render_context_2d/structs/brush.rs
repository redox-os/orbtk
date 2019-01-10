#[derive(Default, Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub position: f64,
    pub color: String,
}

// impl Default for Gradient {
//     fn default() -> Self {
//         Gradient {
//             color_stops: BTreeMap::new()
//         }
//     }
// }

// impl Gradient {
//     pub fn new() -> Self {
//         Gradient::default()
//     }

//     pub fn add_color_stop(&mut self)
// }

#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    SolidColor(String),
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