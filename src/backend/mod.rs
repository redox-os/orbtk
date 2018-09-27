use std::sync::Arc;

use {Rect, Selector, Theme};

pub use self::orbital::*;

mod orbital;

pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub theme: Arc<Theme>,
}

pub trait Renderer {
    fn render(&mut self, theme: &Arc<Theme>);
    fn render_rectangle(&mut self, theme: &Arc<Theme>, bounds: &Rect, selector: &Selector, offset: (i32, i32));
    fn render_text(&mut self, theme: &Arc<Theme>, text: &str, bounds: &Rect, selector: &Selector, offset: (i32, i32));
}

pub trait Backend {
    fn update(&mut self);
    fn bounds(&mut self, bounds: &Rect);
    fn render_context(&mut self) -> RenderContext;
}
