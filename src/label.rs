use super::{Click, Color, Event, Point, Rect, Renderer, Widget};

use std::sync::Arc;

pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub bg: Color,
    pub fg: Color,
    on_click: Option<Arc<Box<Fn(&mut Label, Point)>>>,
    pressed: bool,
}

impl Label {
    pub fn new(rect: Rect, text: &str) -> Self {
        Label {
            rect: rect,
            text: text.to_string(),
            bg: Color::rgb(237, 233, 227),
            fg: Color::rgb(0, 0, 0),
            on_click: None,
            pressed: false,
        }
    }
}

impl Click for Label {
    fn click(&mut self, point: Point){
        let on_click_option = match self.on_click {
            Some(ref on_click) => Some(on_click.clone()),
            None => None
        };

        if let Some(on_click) = on_click_option {
            on_click(self, point);
        }
    }

    fn on_click<T: Fn(&mut Self, Point) + 'static>(mut self, func: T) -> Self {
        self.on_click = Some(Arc::new(Box::new(func)));

        self
    }
}

impl Widget for Label {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.rect(self.rect, self.bg);

        let mut x = 0;
        for c in self.text.chars() {
            if x + 8 <= self.rect.width as isize {
                renderer.char(Point::new(x + self.rect.x, self.rect.y), c, self.fg);
            }
            x += 8;
        }
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                if self.rect.contains(point){
                    if left_button {
                        self.pressed = true;
                    } else {
                        if self.pressed {
                            click = true;
                        }

                        self.pressed = false;
                    }
                } else {
                    if ! left_button {
                        self.pressed = false;
                    }
                }

                if click {
                    let click_point = Point::new(point.x - self.rect.x, point.y - self.rect.y);
                    self.click(click_point);
                }
            },
            _ => ()
        }
    }
}
