use super::{CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;

use std::sync::Arc;

pub struct Label {
    pub rect: CopyCell<Rect>,
    pub text: CloneCell<String>,
    pub bg: Color,
    pub fg: Color,
    text_offset: Point,
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
        self.text.set(text.to_string());
        self
    }

    pub fn text_offset(mut self, x: i32, y: i32) -> Self {
        self.text_offset = Point::new(x, y);
        self
    }
}

impl Click for Label {
    fn emit_click(&self, point: Point){
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
    fn position(self, x: i32, y: i32) -> Self {
        let mut rect = self.rect.get();
        rect.x = x;
        rect.y = y;
        self.rect.set(rect);

        self
    }

    fn size(self, width: u32, height: u32) -> Self {
        let mut rect = self.rect.get();
        rect.width = width;
        rect.height = height;
        self.rect.set(rect);

        self
    }
}

impl Widget for Label {
    fn draw(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();
        renderer.rect(rect, self.bg);

        let text = self.text.borrow();

        let mut x = self.text_offset.x;
        let mut y = self.text_offset.y;
        for c in text.chars() {
            if c == '\n' {
                x = 0;
                y += 16;
            }else{
                if x + 8 <= rect.width as i32 && y + 16 <= rect.height as i32 {
                    renderer.char(Point::new(x, y) + rect.point(), c, self.fg);
                }
                x += 8;
            }
        }
    }

    fn event(&self, event: Event, focused: bool) -> bool {
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
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            },
            _ => ()
        }

        focused
    }
}
