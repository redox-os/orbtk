//! This module contains non visual structures like point, rectangle, color and thickness.

pub use self::constraint::Constraint;
pub use self::label::Label;
pub use self::offset::Offset;
pub use self::padding::Padding;
pub use self::point::Point;
pub use self::rect::*;
pub use self::thickness::Thickness;
pub use self::water_mark::WaterMark;

mod constraint;
mod label;
mod offset;
mod padding;
mod point;
mod rect;
mod thickness;
mod water_mark;

#[cfg(test)]
mod tests;

// pub use self::color::Color;
// pub mod color;
