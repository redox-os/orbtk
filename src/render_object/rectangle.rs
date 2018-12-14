use backend::Renderer;
use properties::{Point, Bounds};
use render_object::RenderObject;
use theme::Selector;
use widget::Context;

pub struct RectangleRenderObject;

impl Into<Box<RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render(&self, renderer: &mut Renderer, context: &mut Context, global_position: &Point) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            if let Ok(bounds) = parent.borrow_property::<Bounds>() {
                bounds.clone()
            } else {
                Bounds::default()
            }
        }  else {
            Bounds::default()
        };

        let theme = context.theme;
        let widget = context.widget();
        
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Bounds>() {
                    renderer.render_rectangle(
                        bounds,
                        &parent_bounds,
                        global_position,
                        theme.uint("border-radius", selector),
                        theme.color("background", selector),
                        theme.uint("border-width", selector),
                        theme.color("border-color", selector),
                        theme.float("opacity", selector),
                    );
                
            }
        }
    }
}
