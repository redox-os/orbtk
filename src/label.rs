use super::{Click, Color, Event, Point, Rect, Renderer, Widget, Window};

use std::cell::{Cell, RefCell};
use std::sync::Arc;

pub struct Label {
    pub rect: Cell<Rect>,
    pub text: RefCell<String>,
    pub bg: Color,
    pub fg: Color,
    on_click: Option<Arc<Fn(&Label, Point)>>,
    pressed: Cell<bool>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            rect: Cell::new(Rect::default()),
            text: RefCell::new(text.to_string()),
            bg: Color::rgb(237, 233, 227),
            fg: Color::rgb(0, 0, 0),
            on_click: None,
            pressed: Cell::new(false),
        }
    }

    pub fn place(self, window: &mut Window) -> Arc<Self> {
        let arc = Arc::new(self);

        window.widgets.push(arc.clone());

        arc
    }
}

impl Click for Label {
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

impl Widget for Label {
    fn draw(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();
        renderer.rect(rect, self.bg);

        let mut x = 0;
        let text = self.text.borrow();
        for c in text.chars() {
            if x + 8 <= rect.width as isize {
                let point = rect.get_point();
                renderer.char(Point::new(x + point.x, point.y), c, self.fg);
            }
            x += 8;
        }
    }

    fn event(&self, event: Event) {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point){
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
                    let rect_point = rect.get_point();
                    let click_point = Point::new(point.x - rect_point.x, point.y - rect_point.y);
                    self.click(click_point);
                }
            },
            _ => ()
        }
    }

    pub fn position(&mut self, x: isize, y: isize) -> &mut Self {
        let mut rect = self.rect.get();
        rect.point = Some(Point {x: x, y: y});
        self.rect.set(rect);
    }
}
