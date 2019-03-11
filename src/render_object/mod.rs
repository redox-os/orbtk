//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::any::Any;

use orbgl_api::Canvas;

use crate::backend::Renderer;
use crate::structs::Point;
use crate::widget::Context;

pub use self::font_icon::FontIconRenderObject;
pub use self::image::ImageRenderObject;
pub use self::rectangle::RectangleRenderObject;
pub use self::text::TextRenderObject;

mod font_icon;
mod image;
mod rectangle;
mod text;

pub trait RenderObject: Any {
    fn render(
        &self,
        canvas: &mut Canvas,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    );
}
