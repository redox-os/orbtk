use crate::{
    proc_macros::IntoRenderObject,
    render_object::*,
    utils::{Brush, Point, Rectangle, Thickness},
};

/// The CursorRenderObject is used to render the `Cursor` widget.
///
/// [`Cursor`]: ../../widgets/struct.Cursor.html
#[derive(Debug, IntoRenderObject)]
pub struct CursorRenderObject;

impl RenderObject for CursorRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point, rtx: &mut RenderContext2D) {
        let (
            bounds,
            background,
            border_width,
            border_brush,
            background_opacity,
            cursor_x,
            selection_width,
            selection_x,
            offset,
        ) = {
            let widget = ctx.widget();
            (
                *widget.get::<Rectangle>("bounds"),
                widget.get::<Brush>("background").clone(),
                *widget.get::<Thickness>("border_width"),
                widget.clone_or_default::<Brush>("border_brush"),
                *widget.get::<f32>("background_opacity"),
                *widget.get::<f64>("cursor_x"),
                *widget.get::<f64>("selection_width"),
                *widget.get::<f64>("selection_x"),
                *widget.get::<f64>("offset"),
            )
        };

        let border_width = border_width.right();

        // background
        rtx.set_alpha(background_opacity);
        rtx.set_fill_style(background);
        rtx.fill_rect(
            global_position.x() + bounds.x() + offset + selection_x - border_width / 2.,
            global_position.y() + bounds.y(),
            selection_width,
            bounds.height(),
        );
        rtx.set_alpha(1.);

        // border
        rtx.set_fill_style(border_brush);
        rtx.fill_rect(
            global_position.x() + bounds.x() + offset + cursor_x - border_width / 2.,
            global_position.y() + bounds.y(),
            border_width,
            bounds.height(),
        );
    }
}
