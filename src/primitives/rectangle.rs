use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::CloneCell;
use draw::draw_box;
use rect::{Rect};
use point::Point;
use thickness::Thickness;
use theme::{Selector, Theme};
use traits::{Place, Style};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

pub struct Rectangle {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    pub selector: CloneCell<Selector>,
}

impl Rectangle {
    pub fn new() -> Arc<Self> {
        Arc::new(Rectangle {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            selector: CloneCell::new(Selector::new(Some("Rectangle"))),
        })
    }
}

impl Place for Rectangle {}

impl Style for Rectangle {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for Rectangle {
    fn name(&self) -> &str {
        "Rectangle"
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
        draw_box(renderer, self.rect().get(), theme, &self.selector().get());
    }

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }
}