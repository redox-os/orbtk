use super::{Color, Event, Rect, Renderer};

use std::any::Any;
use std::cell::Cell;

pub struct WidgetCore {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
}

impl WidgetCore {
    pub fn new() -> Self {
        WidgetCore {
            rect: Cell::new(Rect::default()),
            bg: Color::white(),
            fg: Color::black(),
        }
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }
}

pub trait Widget : Any {
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
}
