use {Rect, Selector};

pub use self::orbital::*;

mod orbital;

pub trait Backend {
    fn render(&mut self);
    fn update(&mut self);
    fn render_rectangle(&mut self, bounds: &Rect, selector: &Selector);
    fn render_text(&mut self, text: &str, bounds: &Rect, selector: &Selector);
    fn bounds(&mut self, bounds: &Rect);
}
