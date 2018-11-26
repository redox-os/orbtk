//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use backend::Renderer;
use theme::Theme;
use widget::WidgetContainer;
use structs::Point;

pub use self::rectangle::*;
pub use self::text::*;

mod rectangle;
mod text;

pub trait RenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        offset: &Point,
        global_position: &Point,
    );
}
