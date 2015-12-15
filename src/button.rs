use super::{Click, CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};

use std::sync::Arc;

pub struct Button {
    pub rect: CopyCell<Rect>,
    pub text: CloneCell<String>,
    pub bg_up: Color,
    pub bg_down: Color,
    pub fg: Color,
    on_click: Option<Arc<Fn(&Button, Point)>>,
    pressed: CopyCell<bool>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Button {
            rect: CopyCell::new(Rect::default()),
            text: CloneCell::new(text.to_string()),
            bg_up: Color::rgb(220, 222, 227),
            bg_down: Color::rgb(203, 205, 210),
            fg: Color::rgb(0, 0, 0),
            on_click: None,
            pressed: CopyCell::new(false),
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

impl Place for Button {
    fn position(mut self, x: isize, y: isize) -> Self {
        let mut rect = self.rect.get();
        rect.x = x;
        rect.y = y;
        self.rect.set(rect);

        self
    }

    fn size(mut self, width: usize, height: usize) -> Self {
        let mut rect = self.rect.get();
        rect.width = width;
        rect.height = height;
        self.rect.set(rect);

        self
    }
}

impl Widget for Button {
    fn draw(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();

        if self.pressed.get() {
            renderer.rect(rect, self.bg_down);
        } else {
            renderer.rect(rect, self.bg_up);
        }

        let mut x = 0;
        let text = self.text.borrow();
        for c in text.chars() {
            if x + 8 <= rect.width as isize {
                renderer.char(Point::new(x + rect.x, rect.y), c, self.fg);
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
                    let click_point = Point::new(point.x - rect.x, point.y - rect.y);
                    self.click(click_point);
                }
            },
            _ => ()
        }
    }
}
