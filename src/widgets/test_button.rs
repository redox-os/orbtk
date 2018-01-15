use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::{CheckSet, CloneCell};
use draw::draw_box;
use event::Event;
use point::Point;
use rect::Rect;
use theme::{Selector, Theme};
use traits::{Click, Place, Style, Text};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

 use primitives::Rectangle;
use primitives::Text as TextWidget;

pub struct TestButton {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    children: RefCell<Vec<Arc<Widget>>>,
    pub selector: CloneCell<Selector>,
    pub text: CloneCell<String>,
    pub text_offset: Cell<Point>,
    click_callback: RefCell<Option<Arc<Fn(&TestButton, Point)>>>,
    hover: Cell<bool>,
    pressed: Cell<bool>,
}

impl TestButton {
    pub fn new() -> Arc<Self> {
        


        let text = TextWidget::new();
        text.text("Button");

        let border = Rectangle::new();
        border.size(50, 28);
        (*border.children().borrow_mut()).push(text);

        let children: RefCell<Vec<Arc<Widget>>> =  RefCell::new(vec![border]);

        Arc::new(TestButton {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children,
            selector: CloneCell::new(Selector::new(Some("TestButton"))),
            text: CloneCell::new(String::new()),
            text_offset: Cell::new(Point::default()),
            click_callback: RefCell::new(None),
            hover: Cell::new(false),
            pressed: Cell::new(false),
        })
    }
}

impl Click for TestButton {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for TestButton {}

impl Text for TestButton {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Style for TestButton {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for TestButton {
    fn name(&self) -> &str {
        "TestButton"
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
        let mut selector = self.selector
            .get()
            .with_pseudo_class(if self.pressed.get() {
                "active"
            } else {
                "inactive"
            });

        if self.hover.get() {
            selector = selector.with_pseudo_class("hover");
        }

        let rect = self.rect.get();

        let w = rect.width as i32;
        let h = rect.height as i32;

        draw_box(renderer, rect, theme, &selector);

        let text = self.text.borrow();

        let mut point = self.text_offset.get();
        for c in text.chars() {
            if c == '\n' {
                point.x = self.text_offset.get().x;
                point.y += 16;
            } else {
                if point.x + 8 <= w && point.y + 16 <= h {
                    renderer.char(
                        point.x + rect.x,
                        point.y + rect.y,
                        c,
                        theme.color("color", &selector),
                    );
                }
                point.x += 8;
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse {
                point,
                left_button,
                ..
            } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if self.hover.check_set(true) {
                        *redraw = true;
                    }

                    if left_button {
                        if self.pressed.check_set(true) {
                            *redraw = true;
                        }
                    } else {
                        if self.pressed.check_set(false) {
                            click = true;
                            *redraw = true;
                        }
                    }
                } else {
                    if self.hover.check_set(false) {
                        *redraw = true;
                    }

                    if self.pressed.check_set(false) {
                        *redraw = true;
                    }
                }

                if click {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
            _ => (),
        }

        focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
