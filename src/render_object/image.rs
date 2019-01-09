use orbclient::Renderer as OrbRenderer;

use crate::{
    core::Renderer,
    properties::{Bounds, Canvas, Image, Point},
    render_object::RenderObject,
    widget::Context,
};

pub struct ImageRenderObject;

impl Into<Box<dyn RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render(
        &self,
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

        if let Ok(bounds) = widget.borrow_property::<Bounds>() {
            if let Ok(image) = widget.borrow_property::<Image>() {
                renderer.render_image(image.data(), bounds, &parent_bounds, global_position);
            }

            if let Ok(canvas) = widget.borrow_property::<Canvas>() {
                renderer.render_image(&canvas.data, bounds, &parent_bounds, global_position);
            }
        }
    }
}
