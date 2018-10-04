use std::any::Any;
use std::sync::Arc;

use { Rect, Renderer, Selector, Theme};

pub use self::rectangle::*;
pub use self::text::*;

mod rectangle;
mod text;

pub trait RenderObject: Send + Sync {
    fn render(
        &self,
        bounds: &Rect,
        selector: &Selector,
        renderer: &mut Renderer,
        offset: (i32, i32),
        theme: &Arc<Theme>,
        content: Option<Arc<Any + Send + Sync>>,
    );
}

pub struct RenderContainer {
    pub bounds: Rect,
    pub selector: Selector,
    pub render_object: Arc<RenderObject>,
    pub offset: (i32, i32),
    pub content: Option<Arc<Any + Send + Sync>>,
}
