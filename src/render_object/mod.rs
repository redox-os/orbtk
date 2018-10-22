use backend::Renderer;
use theme::Theme;
use widget::WidgetContainer;

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
        boundery: (u32, u32),
        offset: (i32, i32),
    );
}
