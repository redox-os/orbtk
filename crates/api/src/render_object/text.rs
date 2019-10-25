use crate::{
    prelude::*,
    utils::{Point, Rectangle, String16},
};

/// Used to render a text.
pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let (bounds, text, foreground, font, font_size) = {
            let widget = context.widget();
            let text = widget.clone::<String16>("text");

            let txt = {
                if !text.is_empty() {
                    text.clone()
                } else {
                    widget.clone_or_default::<String16>("water_mark")
                }
            };
            (
                widget.get::<Rectangle>("bounds").clone(),
                txt.to_string(),
                widget.get::<Brush>("foreground").0.clone(),
                widget.get::<String>("font").clone(),
                *widget.get::<f64>("font_size"),
            )
        };

        if bounds.width == 0.0
            || bounds.height == 0.0
            || foreground.is_transparent()
            || font_size == 0.0
            || text.is_empty()
        {
            return;
        }

        if !text.is_empty() {
            context.render_context_2_d().begin_path();
            context.render_context_2_d().set_font_family(font);
            context.render_context_2_d().set_font_size(font_size);
            context.render_context_2_d().set_fill_style(foreground);

            context.render_context_2_d().fill_text(
                &text,
                global_position.x + bounds.x,
                global_position.y + bounds.y,
            );
            context.render_context_2_d().close_path();
        }
    }
}
