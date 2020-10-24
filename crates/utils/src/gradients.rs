use crate::{Angle, Color, OnLinePos, OnPlanePos, Point, RelativeDir};

/// Describes a position on a colorful gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GradientStop {
    pub pos: Option<OnLinePos>,
    pub color: Color,
}

impl GradientStop {
    pub fn new(pos: Option<OnLinePos>, color: Color) -> GradientStop {
        GradientStop { pos, color }
    }
}

impl From<(OnLinePos, Color)> for GradientStop {
    fn from(d: (OnLinePos, Color)) -> Self {
        Self {
            pos: Some(d.0),
            color: d.1,
        }
    }
}

impl From<(f64, Color)> for GradientStop {
    fn from(d: (f64, Color)) -> Self {
        Self {
            pos: Some(OnLinePos::from_unit_percent(d.0)),
            color: d.1,
        }
    }
}

impl From<Color> for GradientStop {
    fn from(color: Color) -> Self {
        Self { pos: None, color }
    }
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
    /// Defines the linear gradient using an angle and a displacement from the center of the target figure.
    Angle {
        angle: Angle,
        // Defines a displacement from the center of the target shape.
        displacement: OnPlanePos,
    },
    // Defines a gradient as one that crosses the figure in a given direction.
    Direction {
        direction: RelativeDir,
        displacement: OnPlanePos,
    },
}

impl LinearGradientCoords {
    /// Creates a `LinearGradientCoords` from its ends.
    pub fn from_ends(start: impl Into<Point>, end: impl Into<Point>) -> LinearGradientCoords {
        LinearGradientCoords::Ends {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Creates a `LinearGradientCoords` from its angle.
    pub fn from_angle(angle: Angle) -> LinearGradientCoords {
        LinearGradientCoords::Angle {
            angle,
            displacement: OnPlanePos::default(),
        }
    }

    /// Sets the displacement if `self` is defined by its angle, otherwise, this does nothing.
    pub fn with_displacement(&mut self, displacement: impl Into<OnPlanePos>) -> Self {
        if let LinearGradientCoords::Angle {
            angle: _,
            displacement: disp,
        } = self
        {
            *disp = displacement.into();
        }
        *self
    }
}

impl Default for LinearGradientCoords {
    fn default() -> LinearGradientCoords {
        LinearGradientCoords::Direction {
            direction: RelativeDir::Top,
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
    // Radial(RadialGradient),
}

impl Default for GradientKind {
    fn default() -> GradientKind {
        GradientKind::Linear(LinearGradientCoords::default())
    }
}
