use crate::{prelude::*, render::Image, utils::*};

/// Used to render an image.
pub struct ImageRenderObject;

impl Into<Box<dyn RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (bounds, mut image) = {
            let widget = ctx.widget();
            (
                widget.clone::<Rectangle>("bounds"),
                widget.try_clone::<Image>("image"),
            )
        };

        if let Some(image) = &mut image {
            ctx.render_context_2_d()
                .draw_image(image, bounds.x + global_position.x, bounds.y + global_position.y);
        }
    }
}
