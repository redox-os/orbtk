use orbgl_api::Canvas;

use crate::{prelude::*, backend::Renderer};

pub struct FontIconRenderObject;

impl Into<Box<dyn RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
    fn render(
        &self,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            parent.clone_or_default::<Bounds>()
        } else {
            Bounds::default()
        };

        let (bounds, icon, icon_brush, icon_font, icon_size) = {
            let widget = context.widget();
            (widget.clone::<Bounds>(), widget.clone::<FontIcon>(),  widget.get::<IconBrush>().0.clone(),  widget.get::<IconFont>().0.clone(), widget.get::<IconSize>().0 as u32)
        };

        if !icon.0.is_empty() {
            context.renderer().render_text(
                &icon.0,
                &bounds,
                &parent_bounds,
                global_position,
                icon_size,
                icon_brush.into(),
                &icon_font.0,
            );
        }
    }
}
