pub use orbclient::Color;

pub use self::border::*;
pub use self::brush::*;
pub use self::dirty_size::DirtySize;
pub use self::point::Point;
pub use self::rect::*;
pub use self::thickness::Thickness;

mod border;
mod brush;
mod dirty_size;
mod point;
mod rect;
mod thickness;

// todo:  documentation
pub trait Spacer {
    /// Gets left.
    fn left(&self) -> f64;

    /// Sets left.
    fn set_left(&mut self, left: f64);

    /// Gets top.
    fn top(&self) -> f64;

    /// Sets top.
    fn set_top(&mut self, top: f64);

    /// Gets right.
    fn right(&self) -> f64;

    /// Sets right.
    fn set_right(&mut self, right: f64);

    /// Gets bottom.
    fn bottom(&self) -> f64;

    /// Sets bottom.
    fn set_bottom(&mut self, bottom: f64);

    /// Gets thickness.
    fn thickness(&self) -> Thickness;

    /// Sets thickness.
    fn set_thickness<T: Into<Thickness>>(&mut self, thickness: T);
}

#[cfg(test)]
mod tests;
