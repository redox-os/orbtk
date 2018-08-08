use std::sync::Arc;

use theme::{Selector, Theme};
use super::{CloneCell, Rect};

use orbclient::Renderer;

use drawable::Drawable;

pub struct TextDrawable {
    text: String,
    selector: CloneCell<Selector>,
}

impl TextDrawable {
    pub fn new(text: &str, selector: Selector) -> Arc<Self> {
        Arc::new(TextDrawable {
            text: String::from(text),
            selector: CloneCell::new(selector),
        })
    }
}

impl Drawable for TextDrawable {
    fn draw(&self, rect: &Rect, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        let mut current_rect = rect.clone();
        let x = rect.x;
        let selector = &self.selector.get();
        let text = self.text.clone();

        for c in text.chars() {
            if c == '\n' {
                current_rect.x = x;
                current_rect.y += 16;
            } else {
                if current_rect.x + 8 <= rect.x + rect.width as i32
                    && current_rect.y + 16 <= rect.y + rect.height as i32
                {
                    renderer.char(
                        current_rect.x,
                        current_rect.y,
                        c,
                        theme.color("color", selector),
                    );
                }
                current_rect.x += 8;
            }
        }
    }
}