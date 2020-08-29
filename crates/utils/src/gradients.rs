use crate::{Color, Direction, OnLinePos, OnPlanePos, Point};

/// Describes a position on a colorful gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub pos: Option<OnLinePos>,
    pub color: Color,
}

/// Describes the coordinates of a colorful linear gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LinearGradientCoords {
    /// Defines the linear gradient by point A to point B.
    Ends { start: Point, end: Point },
}

impl Default for LinearGradientCoords {
    fn default() -> LinearGradientCoords {
        LinearGradientCoords::Direction {
            direction: Direction::ToTop,
            displacement: OnPlanePos::default(),
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GradientKind {
    Linear(LinearGradientCoords),
//    Radial(RadialGradient),
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
