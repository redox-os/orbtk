use crate::{prelude::*, utils::*};

pub struct FontIconRenderObject;

impl Into<Box<dyn RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            parent.clone_or_default::<Bounds>()
        } else {
            Bounds::default()
        };

        let (bounds, icon, icon_brush, icon_font, icon_size) = {
            let widget = context.widget();
            (
                widget.get::<Bounds>().0,
                widget.clone::<FontIcon>().0,
                widget.get::<IconBrush>().0.clone(),
                widget.get::<IconFont>().0.clone(),
                widget.get::<IconSize>().0,
            )
        };

        if !icon.is_empty() {
            context.render_context_2_d().set_font_family(icon_font);
            context.render_context_2_d().set_font_size(icon_size);
            context.render_context_2_d().set_fill_style(icon_brush);

            context.render_context_2_d().fill_text(
                &icon,
                global_position.x + bounds.x,
                global_position.y + bounds.y,
                None,
            );
            context.render_context_2_d().close_path();
        }
    }
}
