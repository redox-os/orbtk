use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle},
};

#[derive(Debug, IntoRenderObject)]
pub struct FontIconRenderObject;

impl RenderObject for FontIconRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (bounds, icon, icon_brush, icon_font, icon_size) = {
            let widget = ctx.widget();
            (
                *widget.get::<Rectangle>("bounds"),
                widget.clone::<String>("icon"),
                widget.get::<Brush>("icon_brush").clone(),
                widget.get::<String>("icon_font").clone(),
                *widget.get::<f64>("icon_size"),
            )
        };

        if bounds.width() == 0.0
            || bounds.height() == 0.0
            || icon_brush.is_transparent()
            || icon_size == 0.0
            || icon.is_empty()
        {
            return;
        }

        if !icon.is_empty() {
            ctx.render_context_2_d().begin_path();
            ctx.render_context_2_d().set_font_family(icon_font);
            ctx.render_context_2_d().set_font_size(icon_size);
            ctx.render_context_2_d().set_fill_style(icon_brush);

            ctx.render_context_2_d().fill_text(
                &icon,
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
            );
            ctx.render_context_2_d().close_path();
        }
    }
}
