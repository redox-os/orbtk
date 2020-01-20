use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::CloneCell;
use event::Event;
use rect::Rect;
use point::Point;
use thickness::Thickness;
use theme::{Selector, Theme};
use traits::{Place, Style};
use widgets::{HorizontalPlacement, VerticalPlacement, Widget};

pub struct TextWidget {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    pub selector: CloneCell<Selector>,
    pub text: CloneCell<String>,
}

impl TextWidget {
    pub fn new() -> Arc<Self> {
        let text_widget = Arc::new(TextWidget {
            rect: Cell::new(Rect::new(0, 0, 0, 16)),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            selector: CloneCell::new(Selector::new(Some("Text"))),
            text: CloneCell::new(String::new()),
        });

        let text_widget_clone = text_widget.clone();
        text_widget.text.on_changed(move |value: String| {
            text_widget_clone.adjust_width(value.len() as u32);
        });

        text_widget
    }

    pub fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self.adjust_width(self.text.get().len() as u32);
        self
    }

    pub fn inner_text(&self) -> &CloneCell<String> {
        &self.text
    }

    fn adjust_width(&self, text_len: u32) {
        let mut rect = self.rect.get();
        rect.width = text_len * 8;
        self.rect.set(rect);
    }
}

impl Place for TextWidget {}

impl Style for TextWidget {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for TextWidget {
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

    fn margin(&self) -> &Cell<Thickness> {
        &self.margin
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn draw(&self, renderer: &mut dyn Renderer, _focused: bool, theme: &Theme) {
        let rect = self.rect().get();
        let mut current_rect = self.rect().get();
        let x = rect.x;
        let selector = &self.selector().get();
        let text = self.text.get();

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

    fn event(&self, _event: Event, _focused: bool, _redraw: &mut bool, _caught: &mut bool) -> bool {
        _focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }
}
