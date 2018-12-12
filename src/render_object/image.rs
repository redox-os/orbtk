use orbclient::Renderer as OrbRenderer;
use orbimage;

use backend::Renderer;
use properties::{Bounds, Point};
use render_object::RenderObject;
use theme::Selector;
use widget::Context;

pub struct ImageRenderObject;

impl Into<Box<RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render(&self, renderer: &mut Renderer, context: &mut Context, global_position: &Point) {
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
            if let Ok(image) = widget.borrow_property::<orbimage::Image>() {
                renderer.render_image(image.data(), bounds, &parent_bounds, global_position);
            }
        }
    }
}
