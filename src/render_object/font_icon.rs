use orbgl_api::Canvas;

use crate::{
    backend::Renderer,
    properties::{Bounds, FontIcon, IconBrush, IconFont, IconSize},
    render_object::RenderObject,
    structs::Point,
    widget::Context,
};

pub struct FontIconRenderObject;

impl Into<Box<dyn RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
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
        let icon = widget.get_property::<FontIcon>();

        if !icon.0.is_empty() {
            renderer.render_text(
                &icon.0,
                &widget.get_property::<Bounds>(),
                &parent_bounds,
                global_position,
                widget.get_property::<IconSize>().0 as u32,
                widget.get_property::<IconBrush>().into(),
                &widget.get_property::<IconFont>().0,
            );
        }
    }
}
