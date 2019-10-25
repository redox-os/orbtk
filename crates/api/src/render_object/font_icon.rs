use crate::{
    prelude::*,
    utils::{Point, Rectangle},
};

pub struct FontIconRenderObject;

impl Into<Box<dyn RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let (bounds, icon, icon_brush, icon_font, icon_size) = {
            let widget = context.widget();
            (
                widget.get::<Rectangle>("bounds").clone(),
                widget.clone::<String>("icon"),
                widget.get::<Brush>("icon_brush").0.clone(),
                widget.get::<String>("icon_font").clone(),
                *widget.get::<f64>("icon_size"),
            )
        };

        if bounds.width == 0.0
            || bounds.height == 0.0
            || icon_brush.is_transparent()
            || icon_size == 0.0
            || icon.is_empty()
        {
            return;
        }

        if !icon.is_empty() {
            context.render_context_2_d().begin_path();
            context.render_context_2_d().set_font_family(icon_font);
            context.render_context_2_d().set_font_size(icon_size);
            context.render_context_2_d().set_fill_style(icon_brush);

            context.render_context_2_d().fill_text(
                &icon,
                global_position.x + bounds.x,
                global_position.y + bounds.y,
            );
            context.render_context_2_d().close_path();
        }
    }
}
