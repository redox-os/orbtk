//! This module contains non visual structures like point, rectangle, color and thickness.

pub use self::constraint::Constraint;
pub use self::point::Point;
pub use self::rect::*;
pub use self::thickness::Thickness;

mod constraint;
mod point;
mod rect;
mod thickness;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
pub use self::color::Color;

#[cfg(target_arch = "wasm32")]
pub mod color;
