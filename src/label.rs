use super::{CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;
use super::cell::CheckSet;

use std::sync::Arc;

pub struct Label {
    pub rect: CopyCell<Rect>,
    pub text: CloneCell<String>,
    pub bg: Color,
    pub fg: Color,
    pub text_offset: Point,
    click_callback: Option<Arc<Fn(&Label, Point)>>,
    pressed: CopyCell<bool>,
}

impl Label {
    pub fn new() -> Self {
        Label {
            rect: CopyCell::new(Rect::default()),
            text: CloneCell::new(String::new()),
            bg: Color::rgb(237, 233, 227),
            fg: Color::rgb(0, 0, 0),
            text_offset: Point::default(),
            click_callback: None,
            pressed: CopyCell::new(false),
        }
    }

    pub fn place(self, window: &mut Window) -> Arc<Self> {
        let arc = Arc::new(self);

        window.widgets.push(arc.clone());

        arc
    }

    pub fn text(self, text: &str) -> Self {
        self.text.set(text.to_owned());
        self
    }

    pub fn text_offset(mut self, x: i32, y: i32) -> Self {
        self.text_offset = Point::new(x, y);
        self
    }
}

impl Click for Label {
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

impl Place for Label {
    fn rect(&self) -> &CopyCell<Rect> {
        &self.rect
    }
}

impl Widget for Label {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        renderer.rect(rect, self.bg);

        let text = self.text.borrow();

        let mut point = self.text_offset;
        for c in text.chars() {
            if c == '\n' {
                point.x = 0;
                point.y += 16;
            } else {
                if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                    renderer.char(point + rect.point(), c, self.fg);
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
