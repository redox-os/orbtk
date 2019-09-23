use crate::{prelude::*, utils::*};

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
                widget.clone::<Bounds>(),
                widget.try_clone::<Image>().clone(),
            )
        };

        if let Some(image) = &mut image {
            context.render_context_2_d().draw_image(
                &mut image.0,
                global_position.x,
                global_position.y,
            );
        }
    }
}
