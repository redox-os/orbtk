use backend::Renderer;
use render_object::RenderObject;
use structs::{Point, Rect};
use theme::{Selector, Theme};
use widget::WidgetContainer;

pub struct RectangleRenderObject;

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        offset: &Point,
        global_position: &Point,
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                if let Ok(parent_bounds) = widget.borrow_parent_property::<Rect>() {
                    renderer.render_rectangle(
                        theme,
                        bounds,
                        parent_bounds,
                        selector,
                        offset,
                        global_position,
                    );
                }
            }
        }
    }
}
