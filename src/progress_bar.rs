use super::{Click, Color, Event, Point, Rect, Renderer, Widget, Window};

use std::cell::Cell;
use std::cmp::{min, max};
use std::sync::Arc;

pub struct ProgressBar {
    pub rect: Rect,
    pub value: Cell<isize>,
    pub minimum: isize,
    pub maximum: isize,
    pub bg: Color,
    pub fg: Color,
    on_click: Option<Arc<Fn(&ProgressBar, Point)>>,
    pressed: Cell<bool>,
}

impl ProgressBar {
    pub fn new(value: isize) -> Self {
        ProgressBar {
            rect: Rect::default(),
            value: Cell::new(value),
            minimum: 0,
            maximum: 100,
            bg: Color::rgb(255, 255, 255),
            fg: Color::rgb(65, 139, 212),
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

impl Click for ProgressBar {
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

impl Widget for ProgressBar {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.rect(self.rect, self.bg);
        let point = self.rect.get_point();
        renderer.rect(Rect::new(
            point.x,
            point.y,
            ((self.rect.width as isize * max(0, min(self.maximum, self.value.get() - self.minimum)))/max(1, self.maximum - self.minimum)) as usize,
            self.rect.height
        ), self.fg);
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
        self.rect.point = Some(Point::new(x, y));
    }
}
