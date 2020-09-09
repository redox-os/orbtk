use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle},
};
use memchr::memchr_iter;
use std::iter;

/// Used to render a text.
#[derive(Debug, IntoRenderObject)]
pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (bounds, text, foreground, font, font_size, offset) = {
            let widget = ctx.widget();
            let text = text(&widget);
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
                txt,
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
        if text.is_empty() {
            return;
        }

        ctx.render_context_2_d().begin_path();
        ctx.render_context_2_d().set_font_family(font);
        ctx.render_context_2_d().set_font_size(font_size);
        ctx.render_context_2_d().set_fill_style(foreground);

        let mut y_disp = 0.0;
        let mut last_ofs = 0;
        for i in memchr_iter(b'\n', text.as_bytes()).chain(iter::once(text.len())) {
            ctx.render_context_2_d().fill_text(
                &text[last_ofs..i],
                global_position.x() + bounds.x() + offset,
                global_position.y() + bounds.y() + y_disp,
            );
            y_disp += font_size * 1.15; // TODO: Make the space between lines customizable
            last_ofs = i + 1; // + 1 to skip the end of line character
        }

        ctx.render_context_2_d().close_path();
    }
}

fn text(widget: &WidgetContainer) -> String {
    if let Some(localizable) = widget.try_get::<bool>("localizable") {
        if *localizable {
            if let Some(localized_text) = widget.try_get::<String>("localized_text") {
                if !localized_text.is_empty() {
                    return localized_text.clone();
                }
            }
        }
    }

    if let Some(text) = widget.try_get::<String>("text") {
        return text.clone();
    }

    String::default()
}
