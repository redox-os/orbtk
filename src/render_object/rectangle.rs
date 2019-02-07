use crate::{
    backend::Renderer, properties::Bounds, render_object::RenderObject, structs::Point,
    theme::Selector, widget::Context,
};

pub struct RectangleRenderObject;

impl Into<Box<dyn RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let parent_bounds = {
            if let Some(parent) = context.parent_widget() {
                if let Ok(bounds) = parent.borrow_property::<Bounds>() {
                    bounds.clone()
                } else {
                    Bounds::default()
                }
            } else {
                Bounds::default()
            }
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
