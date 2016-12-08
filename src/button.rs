use super::{CloneCell, Color, Event, Placeable, Point, Rect, Renderer, Widget, WidgetCore};
use super::callback::Click;
use super::cell::CheckSet;

use std::cell::Cell;
use std::sync::Arc;

pub struct Button {
    pub core: WidgetCore,
    pub text: CloneCell<String>,
    pub bg_pressed: Color,
    pub text_offset: Point,
    click_callback: Option<Arc<Fn(&Button, Point)>>,
    pressed: Cell<bool>,
}

impl Button {
    pub fn new() -> Self {
        Button {
            core: WidgetCore::new()
                    .bg(Color::rgb(220, 222, 227)),
            text: CloneCell::new(String::new()),
            bg_pressed: Color::rgb(203, 205, 210),
            text_offset: Point::default(),
            click_callback: None,
            pressed: Cell::new(false),
        }
    }

    pub fn text<S: Into<String>>(self, text: S) -> Self {
        self.text.set(text.into());
        self
    }

    pub fn text_offset(mut self, x: i32, y: i32) -> Self {
        self.text_offset = Point::new(x, y);
        self
    }
}

impl Click for Button {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = self.click_callback {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self {
        self.click_callback = Some(Arc::new(func));

        self
    }
}

impl Placeable for Button {}

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
        let b_r = 4;

        // Draw inside corners
        renderer.arc(Point::new(x + b_r, y + b_r), -b_r, 1 << 4 | 1 << 6, bg);
        renderer.arc(Point::new(x + w - b_r, y + b_r), -b_r, 1 << 5 | 1 << 7, bg);
        renderer.arc(Point::new(x + b_r, y + h - 1 - b_r), -b_r, 1 << 0 | 1 << 2, bg);
        renderer.arc(Point::new(x + w - b_r, y + h - 1 - b_r), -b_r, 1 << 1 | 1 << 3, bg);

        // Draw inside rectangles
        renderer.rect(Rect::new(x + b_r, y, (w - b_r * 2) as u32, b_r as u32 + 1), bg);
        renderer.rect(Rect::new(x + b_r, y + h - 1 - b_r, (w - b_r * 2) as u32, b_r as u32 + 1), bg);
        renderer.rect(Rect::new(x, y + b_r + 1, w as u32, (h - 2 - b_r * 2) as u32), bg);

        // Draw outside corners
        renderer.arc(Point::new(x + b_r, y + b_r), b_r, 1 << 4 | 1 << 6, fg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + b_r), b_r, 1 << 5 | 1 << 7, fg);
        renderer.arc(Point::new(x + b_r, y + h - 1 - b_r), b_r, 1 << 0 | 1 << 2, fg);
        renderer.arc(Point::new(x + w - 1 - b_r, y + h - 1 - b_r), b_r, 1 << 1 | 1 << 3, fg);

        // Draw outside rectangles
        renderer.rect(Rect::new(x + b_r + 1, y, (w - 2 - b_r * 2) as u32, 1), fg);
        renderer.rect(Rect::new(x + b_r + 1, y + h - 1, (w - 2 - b_r * 2) as u32, 1), fg);
        renderer.rect(Rect::new(x, y + b_r + 1, 1, (h - 2 - b_r * 2) as u32), fg);
        renderer.rect(Rect::new(x + w - 1, y + b_r + 1, 1, (h - 2 - b_r * 2) as u32), fg);

        let text = self.text.borrow();

        let mut point = self.text_offset;
        for c in text.chars() {
            if c == '\n' {
                point.x = 0;
                point.y += 16;
            } else {
                if point.x + 8 <= w && point.y + 16 <= h {
                    //renderer.char(point + rect.point(), c, fg);
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
