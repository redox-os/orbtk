use super::{Click, CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};

use std::cmp::min;
use std::sync::Arc;

pub struct TextBox {
    pub rect: CopyCell<Rect>,
    pub text: CloneCell<String>,
    pub text_i: CopyCell<usize>,
    pub bg: Color,
    pub fg: Color,
    pub fg_cursor: Color,
    on_click: Option<Arc<Fn(&TextBox, Point)>>,
    pressed: CopyCell<bool>,
}

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            rect: CopyCell::new(Rect::default()),
            text: CloneCell::new(String::new()),
            text_i: CopyCell::new(0),
            bg: Color::rgb(255, 255, 255),
            fg: Color::rgb(0, 0, 0),
            fg_cursor: Color::rgb(128, 128, 128),
            on_click: None,
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
        self.text_i.set(text.len());
        self
    }
}

impl Click for TextBox {
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

impl Place for TextBox {
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

impl Widget for TextBox {
    fn draw(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();

        renderer.rect(rect, self.bg);

        let text_i = self.text_i.get();
        let text = self.text.borrow();
        let mut x = 0;
        for (i, c) in text.char_indices() {
            if x + 8 <= rect.width as isize {
                if i == text_i {
                    renderer.rect(Rect::new(x + rect.x, rect.y, 8, 16), self.fg_cursor);
                }
                renderer.char(Point::new(x, 0) + rect.point(), c, self.fg);
            }
            x += 8;
        }

        if text.len() == text_i {
            if x + 8 <= rect.width as isize {
                renderer.rect(Rect::new(x + rect.x, rect.y, 8, 16), self.fg_cursor);
            }
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
                    let click_point: Point = point - rect.point();
                    {
                        let text = self.text.borrow();
                        let mut x = 0;
                        for (i, _c) in text.char_indices() {
                            if x + 8 <= rect.width as isize && click_point.x >= x && click_point.x < x + 8{
                                self.text_i.set(i);
                            }
                            x += 8;
                        }

                        if x + 8 <= rect.width as isize && click_point.x >= x {
                            self.text_i.set(text.len());
                        }
                    }
                    self.click(click_point);
                }
            },
            Event::Text { c } => {
                let mut text = self.text.borrow_mut();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) {
                    text.insert(text_i, c);
                    self.text_i.set(min(text.char_range_at(text_i).next, text.len()));
                }
            },
            Event::Backspace => {
                let mut text = self.text.borrow_mut();
                let mut text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i > 0 {
                    text_i = min(text.char_range_at_reverse(text_i).next, text.len());
                    if text.is_char_boundary(text_i) && text_i < text.len() {
                        text.remove(text_i);
                        self.text_i.set(min(text_i, text.len()));
                    }
                }
            },
            Event::Delete => {
                let mut text = self.text.borrow_mut();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i < text.len() {
                    text.remove(text_i);
                    self.text_i.set(min(text_i, text.len()));
                }
            },
            Event::LeftArrow => {
                let text = self.text.borrow();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i > 0 {
                    self.text_i.set(min(text.char_range_at_reverse(text_i).next, text.len()));
                }
            },
            Event::RightArrow => {
                let text = self.text.borrow();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i < text.len() {
                    self.text_i.set(min(text.char_range_at(text_i).next, text.len()));
                }
            },
            _ => ()
        }
    }
}
