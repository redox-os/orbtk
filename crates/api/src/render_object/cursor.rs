use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle, Thickness},
};

/// The CursorRenderObject is used to render a text `Cursor`.
#[derive(Debug, IntoRenderObject)]
pub struct CursorRenderObject;

impl RenderObject for CursorRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let (bounds, background, border_width, border_brush) = {
            let widget = ctx.widget();
            (
                widget.clone::<Rectangle>("bounds"),
                widget.get::<Brush>("background").clone(),
                widget.clone_or_default::<Thickness>("border_width"),
                widget.clone_or_default::<Brush>("border_brush"),
            )
        };

        // background
        ctx.render_context_2_d().set_fill_style(background);
        ctx.render_context_2_d().fill_rect(
            global_position.x() + bounds.x(),
            global_position.y() + bounds.y(),
            bounds.width(),
            bounds.height(),
        );

        // border
        let border_width = border_width.right();
        ctx.render_context_2_d().set_fill_style(border_brush);
        ctx.render_context_2_d().fill_rect(
            global_position.x() + bounds.x() + bounds.width() - border_width,
            global_position.y() + bounds.y(),
            border_width,
            bounds.height(),
        );
    }
}
