use crate::{prelude::*, render::Image, utils::*};

/// Used to render an image.
pub struct ImageRenderObject;

impl Into<Box<dyn RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let (_, mut image) = {
            let widget = context.widget();
            (
                widget.clone::<Rectangle>("bounds"),
                widget.try_clone::<Image>("image"),
            )
        };

        if let Some(image) = &mut image {
            context
                .render_context_2_d()
                .draw_image(image, global_position.x, global_position.y);
        }
    }
}
