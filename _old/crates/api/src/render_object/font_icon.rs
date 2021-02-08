use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle},
};

#[derive(Debug, IntoRenderObject)]
pub struct FontIconRenderObject;

impl RenderObject for FontIconRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point, rtx: &mut RenderContext2D) {
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
            rtx.begin_path();
            rtx.set_font_family(icon_font);
            rtx.set_font_size(icon_size);
            rtx.set_fill_style(icon_brush);

            rtx.fill_text(
                &icon,
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
            );
            rtx.close_path();
        }
    }
}
