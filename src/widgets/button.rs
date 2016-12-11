use orbclient::Color;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::{CloneCell, CheckSet};
use event::Event;
use point::Point;
use rect::Rect;
use renderer::Renderer;
use theme::{BUTTON_BACKGROUND, BUTTON_FOREGROUND, BUTTON_SELECTION, BUTTON_BORDER};
use traits::{Click, Place, Text};
use widgets::Widget;

pub struct Button {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
    pub text: CloneCell<String>,
    pub bg_pressed: Color,
    pub fg_border: Color,
    pub text_offset: Cell<Point>,
    click_callback: RefCell<Option<Arc<Fn(&Button, Point)>>>,
    pressed: Cell<bool>,
}

impl Button {
    pub fn new() -> Arc<Self> {
        Arc::new(Button {
            rect: Cell::new(Rect::default()),
            bg: BUTTON_BACKGROUND,
            fg: BUTTON_FOREGROUND,
            text: CloneCell::new(String::new()),
            bg_pressed: BUTTON_SELECTION,
            fg_border: BUTTON_BORDER,
            text_offset: Cell::new(Point::default()),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
        })
    }
}

impl Click for Button {
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

impl Place for Button {}

impl Text for Button {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Widget for Button {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        let w = rect.width as i32;
        let h = rect.height as i32;

        let fg = self.fg;

        let bg = if self.pressed.get() {
            self.bg_pressed
        } else {
            self.bg
        };

        // Border radius
        let b_r = 2;

        renderer.rounded_rect(rect, -b_r, bg);
        renderer.rounded_rect(rect, b_r, self.fg_border);

        let text = self.text.borrow();

        let mut point = self.text_offset.get();
        for c in text.chars() {
            if c == '\n' {
                point.x = 0;
                point.y += 16;
            } else {
                if point.x + 8 <= w && point.y + 16 <= h {
                    renderer.char(point + rect.point(), c, fg);
                }
                point.x += 8;
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
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
                    if !left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
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
}
