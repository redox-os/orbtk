//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::collections::BTreeMap;

use crate::core::Renderer;
use crate::properties::Point;
use crate::widget::Context;

pub use self::font_icon::FontIconRenderObject;
pub use self::image::ImageRenderObject;
pub use self::rectangle::RectangleRenderObject;
pub use self::text::TextRenderObject;

mod font_icon;
mod image;
mod rectangle;
mod text;


// pub trait Shape2DBuilder {
//     fn build(&self) -> Box<Shape2D>;
// }

pub struct RectangleBuilder {}

// zoom, rotate, ...

pub struct CCanvas {}

pub trait RenderObject {
    fn render(
        &self,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    );
}
