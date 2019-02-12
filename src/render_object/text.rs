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
        canvas: &mut Canvas,
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

        let bounds = widget.get_property::<Bounds>();
        let foreground = widget.get_property::<Foreground>();
        let font_size = widget.get_property::<FontSize>();
        let text = widget.get_property::<Text>();
        let font = widget.get_property::<Font>();

        if !text.0.is_empty() {
            renderer.render_text(
                &text.0,
                &bounds,
                &parent_bounds,
                global_position,
                font_size.0 as u32,
                foreground.into(),
                &font.0,
            );
        }
    }
}
