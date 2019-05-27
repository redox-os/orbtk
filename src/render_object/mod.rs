//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::any::Any;

use crate::prelude::*;

pub use self::font_icon::*;
pub use self::image::*;
pub use self::rectangle::*;
pub use self::text::*;

mod font_icon;
mod image;
mod rectangle;
mod text;

pub trait RenderObject: Any {
    fn render(
        &self,
        context: &mut Context<'_>,
        global_position: &Point,
    );
}
