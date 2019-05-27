use crate::prelude::*;

/// Used to render a text.
pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
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
            (widget.clone::<Bounds>(), txt.to_string(), widget.get::<Foreground>().0.clone(), widget.get::<Font>().0.clone(), widget.get::<FontSize>().0 as u32)
        };

        context.renderer().render_text(
            &text,
            &bounds.0,
            &parent_bounds.0,
            global_position,
            font_size,
            foreground.into(),
            &font.0,
        );
    }
}
