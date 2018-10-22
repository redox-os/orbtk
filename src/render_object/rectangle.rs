use backend::Renderer;
use render_object::RenderObject;
use structs::Rect;
use theme::{Selector, Theme};
use widget::WidgetContainer;

pub struct RectangleRenderObject;

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        boundery: (u32, u32),
        offset: (i32, i32),
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                renderer.render_rectangle(theme, bounds, selector, boundery, offset);
            }
        }
    }
}
