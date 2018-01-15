use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::CloneCell;
use event::Event;
use rect::Rect;
use point::Point;
use theme::{Selector, Theme};
use traits::{Place, Style};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

pub struct Text {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<Widget>>>,
    pub selector: CloneCell<Selector>,
    pub text: CloneCell<String>,
}

impl Text {
    pub fn new() -> Arc<Self> {
        Arc::new(Text {
            rect: Cell::new(Rect::new(0, 0, 0, 16)),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(vec![]),
            selector: CloneCell::new(Selector::new(Some("Text"))),
            text: CloneCell::new(String::new()),
        })
    }

    pub fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self.adjust_width();
        self
    }

    fn adjust_width(&self) {
        let mut rect = self.rect.get();
        rect.width =  self.text.get().len() as u32 * 8;
        self.rect.set(rect);
    }
}

impl Place for Text {}

impl Style for Text {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for Text {
    fn name(&self) -> &str {
        "Text"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        let mut rect = self.rect().get();
        let x = rect.x;
        let selector = &self.selector().get();
        let text = self.text.get();
       
        for c in text.chars() {
            if c == '\n' {
                rect.x = x;
                rect.y += 16;
            } else {
                if rect.x + 8 <= rect.width as i32 && rect.y + 16 <= rect.height as i32 {
                    renderer.char(
                        rect.x,
                        rect.y,
                        c,
                        theme.color("color", selector),
                    );
                }
                rect.x += 8;
            }
        }
    }

    fn event(&self, _event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        _focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
