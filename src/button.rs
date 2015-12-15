use super::{Click, Color, Event, Point, Rect, Renderer, Widget, Window};

use std::cell::Cell;
use std::sync::Arc;

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub bg_up: Color,
    pub bg_down: Color,
    pub fg: Color,
    on_click: Option<Arc<Fn(&Button, Point)>>,
    pressed: Cell<bool>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Button {
            rect: Rect::default(),
            text: text.to_string(),
            bg_up: Color::rgb(220, 222, 227),
            bg_down: Color::rgb(203, 205, 210),
            fg: Color::rgb(0, 0, 0),
            on_click: None,
            pressed: Cell::new(false),
        }
    }

    pub fn place(self, window: &mut Window) -> Arc<Self> {
        let rc = Arc::new(self);

        window.widgets.push(rc.clone());

        rc
    }
}

impl Click for Button {
    fn click(&self, point: Point){
        let on_click_option = match self.on_click {
            Some(ref on_click) => Some(on_click.clone()),
            None => None
        };

        if let Some(on_click) = on_click_option {
            on_click(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self {
        self.on_click = Some(Arc::new(func));

        self
    }
}

impl Widget for Button {
    fn draw(&self, renderer: &mut Renderer) {
        if self.pressed.get() {
            renderer.rect(self.rect, self.bg_down);
        } else {
            renderer.rect(self.rect, self.bg_up);
        }

        let mut x = 0;
        for c in self.text.chars() {
            if x + 8 <= self.rect.width as isize {
                let point = self.rect.get_point();
                renderer.char(Point::new(x, 0) + point, c, self.fg);
            }
            x += 8;
        }
    }

    fn event(&self, event: Event) {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                if self.rect.contains(point){
                    if left_button {
                        self.pressed.set(true);
                    } else {
                        if self.pressed.get() {
                            click = true;
                        }

                        self.pressed.set(false);
                    }
                } else {
                    if ! left_button {
                        self.pressed.set(false);
                    }
                }

                if click {
                    let click_point: Point = point - self.rect.get_point();
                    self.click(click_point);
                }
            },
            _ => ()
        }
    }

    pub fn position(&mut self, x: isize, y: isize) -> &mut Self {
        self.rect.point = Some(Point {x: x, y: y});
    }
}
