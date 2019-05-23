use orbgl_api::Canvas;

use crate::{prelude::*, backend::Renderer};

/// Used to render an image.
pub struct ImageRenderObject;

impl Into<Box<dyn RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render(
        &self,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
         let (bounds, mut image) = {
            let widget = context.widget();
            (widget.clone::<Bounds>(), widget.try_clone::<Image>().clone())
        };

        if let Some(image) = &mut image {
            context.canvas().draw_image_with_size(&mut (image.0).0, global_position.x, global_position.y, bounds.width(), bounds.height());
        }
    }
}
