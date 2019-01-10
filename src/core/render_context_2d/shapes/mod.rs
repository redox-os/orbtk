use super::Instruction;

pub use self::image::Image;
pub use self::rectangle::Rectangle;

mod image;
mod rectangle;

pub trait Shape2D {
    fn instructions(&self) -> &[Instruction];
}