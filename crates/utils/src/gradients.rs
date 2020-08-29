use crate::{Color, OnLinePos, Point};

/// Describes a position on a colorful gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub pos: Option<OnLinePos>,
    pub color: Color,
}

impl Default for GradientStop {
    fn default() -> GradientStop {
        GradientStop {
            pos: None,
            color: Color::default(),
        }
    }
}

/// Describes the coordinates of a colorful linear gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LinearGradientCoords {
    /// Defines the linear gradient by point A to point B.
    Ends { start: Point, end: Point },
}

impl Default for LinearGradientCoords {
    fn default() -> LinearGradientCoords {
        LinearGradientCoords::Ends {
            start: Point::default(),
            end: Point::default(),
        }
    }
}

/// Describes a colorful gradient.
#[derive(Clone, PartialEq, Debug)]
pub struct Gradient {
    pub kind: GradientKind,
    pub stops: Vec<GradientStop>,
    pub repeat: bool,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: GradientKind::Linear(LinearGradientCoords::default()),
            stops: vec![
                GradientStop {
                    pos: None,
                    color: Color::rgb(0, 0, 0),
                },
                GradientStop {
                    pos: None,
                    color: Color::rgb(255, 255, 255),
                },
            ],
            repeat: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GradientKind {
    Linear(LinearGradientCoords),
    //    Radial(RadialGradient),
}

impl Default for GradientKind {
    fn default() -> GradientKind {
        GradientKind::Linear(LinearGradientCoords::default())
    }
}
