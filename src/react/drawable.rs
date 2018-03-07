use std::any::Any;
use std::sync::Arc;

use theme::{Selector, Theme};
use super::{CloneCell, Rect};
use draw::draw_box;

use orbclient::Renderer;

pub trait Drawable: Any {
    fn draw(&self, rect: &Rect, _renderer: &mut Renderer, _focused: bool, _theme: &Theme);
}

pub struct TextDrawable {
    text: String,
    selector: CloneCell<Selector>,
}

impl TextDrawable {
    pub fn new(text: &str) -> Arc<Self> {
        Arc::new(TextDrawable {
            text: String::from(text),
            selector: CloneCell::new(Selector::new(Some("text"))),
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

pub struct RectangleDrawable {
    selector: CloneCell<Selector>,
}

impl RectangleDrawable {
    pub fn new() -> Arc<Self> {
        Arc::new(RectangleDrawable {
            selector: CloneCell::new(Selector::new(Some("rectangle"))),
        })
    }
}

impl Drawable for RectangleDrawable {
    fn draw(&self, rect: &Rect, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        draw_box(renderer, rect.clone(), theme, &self.selector.get());
    }
}
