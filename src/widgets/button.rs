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
use widgets::{Widget, WidgetCore};

pub struct Button {
    pub core: WidgetCore,
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
            core: WidgetCore::new(BUTTON_BACKGROUND, BUTTON_FOREGROUND),
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
        &self.core.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();

        let x = rect.x;
        let y = rect.y;
        let w = rect.width as i32;
        let h = rect.height as i32;

        let fg = self.core.fg;

        let bg = if self.pressed.get() {
            self.bg_pressed
        } else {
            self.core.bg
        };

        // Border radius
        let b_r = 2;

        let b_fg = self.fg_border;

        // Draw inside corners
        renderer.arc(Point::new(x + b_r, y + b_r), -b_r, 1 << 4 | 1 << 6, bg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + b_r), -b_r, 1 << 5 | 1 << 7, bg);
        renderer.arc(Point::new(x + b_r, y + h - 1 - b_r), -b_r, 1 << 0 | 1 << 2, bg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + h - 1 - b_r), -b_r, 1 << 1 | 1 << 3, bg);

        // Draw inside rectangles
        renderer.rect(Rect::new(x + b_r, y, (w - 1 - b_r * 2) as u32, b_r as u32 + 1), bg);
        renderer.rect(Rect::new(x + b_r, y + h - 1 - b_r, (w - 1 - b_r * 2) as u32, b_r as u32 + 1), bg);
        renderer.rect(Rect::new(x, y + b_r + 1, w as u32, (h - 2 - b_r * 2) as u32), bg);

        // Draw outside corners
        renderer.arc(Point::new(x + b_r, y + b_r), b_r, 1 << 4 | 1 << 6, b_fg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + b_r), b_r, 1 << 5 | 1 << 7, b_fg);
        renderer.arc(Point::new(x + b_r, y + h - 1 - b_r), b_r, 1 << 0 | 1 << 2, b_fg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + h - 1 - b_r), b_r, 1 << 1 | 1 << 3, b_fg);

        // Draw outside rectangles
        renderer.rect(Rect::new(x + b_r + 1, y, (w - 2 - b_r * 2) as u32, 1), b_fg);
        renderer.rect(Rect::new(x + b_r + 1, y + h - 1, (w - 2 - b_r * 2) as u32, 1), b_fg);
        renderer.rect(Rect::new(x, y + b_r + 1, 1, (h - 2 - b_r * 2) as u32), b_fg);
        renderer.rect(Rect::new(x + w - 1, y + b_r + 1, 1, (h - 2 - b_r * 2) as u32), b_fg);

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

                let rect = self.core.rect.get();
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
