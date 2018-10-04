use std::any::Any;
use std::sync::Arc;

use {Rect, RenderObject, Renderer, Selector, Theme};

pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn render(
       &self,
        bounds: &Rect,
        selector: &Selector,
        renderer: &mut Renderer,
        offset: (i32, i32),
        theme: &Arc<Theme>,
        content: Option<Arc<Any + Send + Sync>>,
    ) {
        if let Some(label) = content {
            if let Ok(label) = label.downcast::<String>() {
                renderer.render_text(&label, &bounds, &selector, offset, &theme);
            }
        }
    }
}
