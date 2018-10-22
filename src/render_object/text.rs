use backend::Renderer;
use render_object::RenderObject;
use structs::Rect;
use theme::{Selector, Theme};
use widget::{Label, WidgetContainer};

pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
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
                if let Ok(label) = widget.borrow_property::<Label>() {
                    renderer.render_text(theme, &label.0, bounds, selector, boundery, offset);
                }
            }
        }
    }
}
