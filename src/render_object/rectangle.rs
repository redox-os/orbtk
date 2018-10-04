use std::any::Any;
use std::sync::Arc;

use {Rect, RenderObject, Renderer, Theme, Selector};

pub struct RectangleRenderObject;

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        bounds: &Rect,
        selector: &Selector,
        renderer: &mut Renderer,
        offset: (i32, i32),
        theme: &Arc<Theme>,
        _content: Option<Arc<Any + Send + Sync>>,
    ) {
        renderer.render_rectangle(&bounds, &selector, offset, &theme);
    }
}
