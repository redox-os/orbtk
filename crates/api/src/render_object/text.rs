use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle},
};

/// Used to render a text.
#[derive(Debug, IntoRenderObject)]
pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (bounds, text, foreground, font, font_size, offset) = {
            let widget = ctx.widget();
            let text = widget.clone::<String>("text");
            let offset = *widget.get::<f64>("offset");

            let txt = {
                if !text.is_empty() {
                    text
                } else {
                    widget.clone_or_default::<String>("water_mark")
                }
            };
            (
                *widget.get::<Rectangle>("bounds"),
                txt.to_string(),
                widget.get::<Brush>("foreground").clone(),
                widget.get::<String>("font").clone(),
                *widget.get::<f64>("font_size"),
                offset,
            )
        };

        if bounds.width() == 0.0
            || bounds.height() == 0.0
            || foreground.is_transparent()
            || font_size == 0.0
            || text.is_empty()
        {
            return;
        }

        if !text.is_empty() {
            ctx.render_context_2_d().begin_path();
            ctx.render_context_2_d().set_font_family(font);
            ctx.render_context_2_d().set_font_size(font_size);
            ctx.render_context_2_d().set_fill_style(foreground);

            ctx.render_context_2_d().fill_text(
                &text,
                global_position.x() + bounds.x() + offset,
                global_position.y() + bounds.y(),
            );
            ctx.render_context_2_d().close_path();
        }
    }
}
