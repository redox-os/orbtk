use orbgl_api::Canvas;

use crate::{
    backend::Renderer,
    properties::{Bounds, Font, FontSize, Foreground, Text, WaterMark},
    render_object::RenderObject,
    structs::{Color, Point},
    theme::Selector,
    widget::Context,
};

pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        _canvas: &mut Canvas,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            if let Ok(bounds) = parent.borrow_property::<Bounds>() {
                bounds.clone()
            } else {
                Bounds::default()
            }
        } else {
            Bounds::default()
        };

        let widget = context.widget();
        let text = widget.get_property::<Text>();

        if !text.0.is_empty() {
            renderer.render_text(
                &text.0,
                &widget.get_property::<Bounds>(),
                &parent_bounds,
                global_position,
                widget.get_property::<FontSize>().0 as u32,
                widget.get_property::<Foreground>().into(),
                &widget.get_property::<Font>().0,
            );
        }
    }
}
