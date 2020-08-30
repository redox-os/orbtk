use std::f64::consts::PI;
use derive_more::{Add, From, Sub, Mul, Div};

#[derive(Add, Sub, Copy, From, Clone, Debug, PartialEq, Mul, Div)]
pub struct Angle(f64);

impl Angle {
    // TODO: Make this const fns, when the floating point operations on const fns become stable
    pub fn from_radians(radians: f64) -> Angle {
        Angle(radians)
    }

    pub fn from_degrees(degrees: f64) -> Angle {
        Angle(degrees * PI / 180.0)
    }

    pub fn from_turn(turn: f64) -> Angle {
        Angle(turn * PI * 2.0)
    }

    pub fn to_radians(&self) -> f64 {
        self.0
    }

    pub fn to_degrees(&self) -> f64 {
        self.0 * 180.0 / PI
    }

    pub fn to_turn(&self) -> f64 {
        self.0 / PI / 2.0
    }

    pub fn zero() -> Angle {
        Angle(0.0)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::zero()
    }
}