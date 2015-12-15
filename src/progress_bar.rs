use super::{Click, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};

use std::cmp::{min, max};
use std::sync::Arc;

pub struct ProgressBar {
    pub rect: CopyCell<Rect>,
    pub value: CopyCell<isize>,
    pub minimum: isize,
    pub maximum: isize,
    pub bg: Color,
    pub fg: Color,
    on_click: Option<Arc<Fn(&ProgressBar, Point)>>,
    pressed: CopyCell<bool>,
}

impl ProgressBar {
    pub fn new(value: isize) -> Self {
        ProgressBar {
            rect: CopyCell::new(Rect::default()),
            value: CopyCell::new(value),
            minimum: 0,
            maximum: 100,
            bg: Color::rgb(255, 255, 255),
            fg: Color::rgb(65, 139, 212),
            on_click: None,
            pressed: CopyCell::new(false),
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

impl Place for ProgressBar {
    fn position(self, x: isize, y: isize) -> Self {
        let mut rect = self.rect.get();
        rect.x = x;
        rect.y = y;
        self.rect.set(rect);

        self
    }

    fn size(self, width: usize, height: usize) -> Self {
        let mut rect = self.rect.get();
        rect.width = width;
        rect.height = height;
        self.rect.set(rect);

        self
    }
}

impl Widget for ProgressBar {
    fn draw(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();
        renderer.rect(rect, self.bg);
        renderer.rect(Rect::new(
            rect.x,
            rect.y,
            ((rect.width as isize * max(0, min(self.maximum, self.value.get() - self.minimum)))/max(1, self.maximum - self.minimum)) as usize,
            rect.height
        ), self.fg);
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
                    let click_point = Point::new(point.x - rect.x, point.y - rect.y);
                    self.click(click_point);
                }
            },
            _ => ()
        }
    }
}
