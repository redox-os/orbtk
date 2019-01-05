//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use backend::Renderer;
use widget::Context;
use properties::Point;

pub use self::font_icon::FontIconRenderObject;
// pub use self::image::ImageRenderObject;
pub use self::rectangle::RectangleRenderObject;
pub use self::text::TextRenderObject;

mod font_icon;
// mod image;
mod rectangle;
mod text;

pub trait RenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        context: &mut Context,
        global_position: &Point,
    );
}
