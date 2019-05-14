use orbgl_api::Canvas;

use crate::{prelude::*, backend::Renderer};

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
        _canvas: &mut Canvas,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            parent.clone_or_default::<Bounds>()
        } else {
            Bounds::default()
        };

        let widget = context.widget();
        let text = widget.clone::<Text>();

        let txt = {
            if !text.0.is_empty() {
                text.0.clone()
            } else {
                widget.clone_or_default::<WaterMark>().0
            }
        };

        renderer.render_text(
            &txt,
            &widget.get::<Bounds>(),
            &parent_bounds,
            global_position,
            widget.get::<FontSize>().0 as u32,
            widget.clone::<Foreground>().into(),
            &(widget.get::<Font>().0).0,
        );
    }
}
