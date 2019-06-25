use crate::{prelude::*, utils::*};

/// Used to render a text.
pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(&self, context: &mut Context<'_>, global_position: &Point) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            parent.clone_or_default::<Bounds>()
        } else {
            Bounds::default()
        };

        let (bounds, text, foreground, font, font_size) = {
            let widget = context.widget();
            let text = widget.clone::<Text>();

            let txt = {
                if !text.0.is_empty() {
                    text.0.clone()
                } else {
                    widget.clone_or_default::<WaterMark>().0
                }
            };
            (
                widget.get::<Bounds>().0,
                txt.to_string(),
                widget.get::<Foreground>().0.clone(),
                widget.get::<Font>().0.clone(),
                widget.get::<FontSize>().0,
            )
        };

        if !text.is_empty() {
            context.render_context_2_d().save();
            context.render_context_2_d().begin_path();
            context.render_context_2_d().rect(
                global_position.x,
                global_position.y,
                parent_bounds.width(),
                parent_bounds.height(),
            );
            context.render_context_2_d().clip();

            context.render_context_2_d().set_font_family(font);
            context.render_context_2_d().set_font_size(font_size);
            context.render_context_2_d().set_fill_style(foreground);

            context
                .render_context_2_d()
                .set_text_baseline(TextBaseline::Middle);

            context.render_context_2_d().fill_text(
                &text,
                global_position.x + bounds.x,
                global_position.y + bounds.y + bounds.height / 2.0,
                None,
            );
            context.render_context_2_d().close_path();
            context.render_context_2_d().restore();
        }
    }
}
