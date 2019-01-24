pub use orbgl_shapes::structs::*;

pub use self::point::Point;

mod point;

// todo: move to orbgl_shapes and documentation
pub trait Spacer {
    fn left(&self) -> f64;

    fn set_left(&mut self, left: f64);

    fn top(&self) -> f64;

    fn set_top(&mut self, top: f64);

    fn right(&self) -> f64;

    fn set_right(&mut self, right: f64);

    fn bottom(&self) -> f64;

    fn set_bottom(&mut self, bottom: f64);

    fn thickness(&self) -> Thickness;

    fn set_thickness(&mut self, thickness: Thickness);
}
