use std::any::Any;

use theme::{Theme};
use super::{CloneCell, Rect};

use orbclient::Renderer;

pub use self::rectangle_drawable::*;
pub use self::text_drawable::*;

mod rectangle_drawable;
mod text_drawable;

pub trait Drawable: Any {
    fn draw(&self, rect: &Rect, _renderer: &mut Renderer, _focused: bool, _theme: &Theme);
}