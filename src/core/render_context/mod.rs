
use orbgl::Canvas;

use crate::theme::Theme;

pub use self::shapes::*;
pub use self::structs::*;
pub use self::render_context_2d::{RenderContext2D, Instruction2D, FillRule};

mod shapes;
mod structs;
mod render_context_2d;

pub struct RenderContext<'a> {
    pub context_2d: &'a mut Canvas,
    pub theme: &'a Theme,
}