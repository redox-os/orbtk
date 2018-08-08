use std::sync::Arc;

use theme::{Selector, Theme};
use super::{CloneCell, Rect};
use draw::draw_box;
use orbclient::Renderer;

use drawable::Drawable;

pub struct RectangleDrawable {
    selector: CloneCell<Selector>,
}

impl RectangleDrawable {
    pub fn new(selector: Selector) -> Arc<Self> {
        Arc::new(RectangleDrawable {
            selector: CloneCell::new(selector),
        })
    }
}

impl Drawable for RectangleDrawable {
    fn draw(&self, rect: &Rect, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        draw_box(renderer, rect.clone(), theme, &self.selector.get());
    }
}