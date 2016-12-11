use std::any::Any;
use std::cell::Cell;

use color::Color;
use event::Event;
use rect::Rect;
use renderer::Renderer;

pub use self::button::Button;
pub use self::canvas::Canvas;
pub use self::label::Label;
pub use self::menu::{Menu, Action, Separator};
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;

mod button;
mod canvas;
mod label;
mod menu;
mod progress_bar;
mod text_box;

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
    fn rect(&self) -> &Cell<Rect>;
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
}
