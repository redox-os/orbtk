use super::{Click, Color, Event, Point, Rect, Renderer, Widget};

use std::cmp::{min, max};
use std::sync::Arc;

pub struct ProgressBar {
    pub rect: Rect,
    pub value: isize,
    pub minimum: isize,
    pub maximum: isize,
    pub bg: Color,
    pub fg: Color,
    on_click: Option<Arc<Box<Fn(&mut ProgressBar, Point)>>>,
    pressed: bool,
}

impl ProgressBar {
    pub fn new(rect: Rect, value: isize) -> Box<Self> {
        Box::new(ProgressBar {
            rect: rect,
            value: value,
            minimum: 0,
            maximum: 100,
            bg: Color::rgb(255, 255, 255),
            fg: Color::rgb(65, 139, 212),
            on_click: None,
            pressed: false,
        })
    }
}

impl Click for ProgressBar {
    fn click(&mut self, point: Point){
        let on_click_option = match self.on_click {
            Some(ref on_click) => Some(on_click.clone()),
            None => None
        };

        if let Some(on_click) = on_click_option {
            on_click(self, point);
        }
    }

    fn on_click(&mut self, func: Box<Fn(&mut Self, Point)>) -> &mut Self {
        self.on_click = Some(Arc::new(func));

        self
    }
}

impl Widget for ProgressBar {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.rect(self.rect, self.bg);
        renderer.rect(Rect::new(
            self.rect.x,
            self.rect.y,
            ((self.rect.width as isize * max(0, min(self.maximum, self.value - self.minimum)))/max(1, self.maximum - self.minimum)) as usize,
            self.rect.height
        ), self.fg);
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
