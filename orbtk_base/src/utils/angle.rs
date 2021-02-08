use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

pub const TAU: f64 = 6.283_185_307_179_586_f64;

/// The OrbTk way to handle angles
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Angle(f64);

impl Angle {
    // TODO: Make this const fns, when the floating point operations on const fns become stable
    pub fn from_radians(radians: f64) -> Angle {
        Angle(radians)
    }

    pub fn from_degrees(degrees: f64) -> Angle {
        Angle(degrees * PI / 180.0)
    }

    /// Takes a number between 0.0 and 1.0 where 0.0 represents 0 degrees and 1.0 360 degrees
    pub fn from_turn(turn: f64) -> Angle {
        Angle(turn * TAU)
    }

    pub fn to_radians(&self) -> f64 {
        self.0
    }

    pub fn to_degrees(&self) -> f64 {
        self.0 * 180.0 / PI
    }

    /// Gives a number between 0.0 and 1.0 where 0.0 represents 0 degrees and 1.0 360 degrees
    pub fn to_turn(&self) -> f64 {
        self.0 / TAU
    }

    /// Creates a `Angle` with a value of 0.0
    pub fn zero() -> Angle {
        Angle(0.0)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<f64> for Angle {
    fn from(v: f64) -> Self {
        Angle(v)
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 -= rhs.0;
        self
    }
}

impl Mul for Angle {
    type Output = Angle;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 *= rhs.0;
        self
    }
}

impl Div for Angle {
    type Output = Angle;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.0 /= rhs.0;
        self
    }
}
