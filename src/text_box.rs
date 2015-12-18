use super::{Click, CloneCell, Color, CopyCell, Enter, Event, Place, Point, Rect, Renderer, Widget, Window};

use std::cmp::min;
use std::sync::Arc;

pub struct TextBox {
    pub rect: CopyCell<Rect>,
    pub text: CloneCell<String>,
    pub text_i: CopyCell<usize>,
    pub bg: Color,
    pub fg: Color,
    pub fg_cursor: Color,
    click_callback: Option<Arc<Fn(&TextBox, Point)>>,
    enter_callback: Option<Arc<Fn(&TextBox)>>,
    pressed: CopyCell<bool>,
    focused: CopyCell<bool>,
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
            click_callback: None,
            enter_callback: None,
            pressed: CopyCell::new(false),
            focused: CopyCell::new(false),
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
    fn trigger_click(&self, point: Point){
        if let Some(ref click_callback) = self.click_callback {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self {
        self.click_callback = Some(Arc::new(func));

        self
    }
}

impl Enter for TextBox {
    fn trigger_on_enter(&self) {
        if let Some(ref enter_callback) = self.enter_callback {
            enter_callback(self)
        }
    }

    fn on_enter<T: Fn(&Self) + 'static>(mut self, func: T) -> Self {
        self.enter_callback = Some(Arc::new(func));

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
        let mut y = 0;
        for (i, c) in text.char_indices() {
            if c == '\n' {
                if i == text_i && self.focused.get() && x + 8 <= rect.width as isize && y + 16 <= rect.height as isize {
                    renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
                }

                x = 0;
                y += 16;
            } else {
                if x + 8 <= rect.width as isize && y + 16 <= rect.height as isize {
                    if i == text_i && self.focused.get() {
                        renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
                    }
                    renderer.char(Point::new(x, y) + rect.point(), c, self.fg);
                }

                x += 8;
            }
        }

        if text.len() == text_i && self.focused.get() && x + 8 <= rect.width as isize && y + 16 <= rect.height as isize {
            renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
        }
    }

    fn event(&self, event: Event, mut focused: bool) -> bool {
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
                    focused = true;

                    let click_point: Point = point - rect.point();
                    {
                        let text = self.text.borrow();

                        let mut new_text_i = None;

                        let mut x = 0;
                        let mut y = 0;
                        for (i, c) in text.char_indices() {
                            if c == '\n' {
                                if x + 8 <= rect.width as isize && click_point.x >= x && y + 16 <= rect.height as isize && click_point.y >= y && click_point.y < y + 16 {
                                    new_text_i = Some(i);
                                    break;
                                }
                                x = 0;
                                y += 16;
                            }else{
                                if x + 8 <= rect.width as isize && click_point.x >= x && click_point.x < x + 8 && y + 16 <= rect.height as isize && click_point.y >= y && click_point.y < y + 16 {
                                    new_text_i = Some(i);
                                    break;
                                }
                                x += 8;
                            }
                        }

                        if new_text_i.is_none() && x + 8 <= rect.width as isize && click_point.x >= x &&  y + 16 <= rect.height as isize && click_point.y >= y || click_point.y >= y + 16 {
                            new_text_i = Some(text.len());
                        }

                        if let Some(text_i) = new_text_i {
                            self.text_i.set(text_i);
                        }
                    }
                    self.trigger_click(click_point);
                }
            },
            Event::Text { c } => if focused {
                let mut text = self.text.borrow_mut();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) {
                    text.insert(text_i, c);
                    self.text_i.set(min(text.char_range_at(text_i).next, text.len()));
                }
            },
            Event::Enter => if focused {
                if self.enter_callback.is_some() {
                    self.trigger_on_enter();
                } else {
                    let mut text = self.text.borrow_mut();
                    let text_i = self.text_i.get();
                    if text.is_char_boundary(text_i) {
                        text.insert(text_i, '\n');
                        self.text_i.set(min(text.char_range_at(text_i).next, text.len()));
                    }
                }
            },
            Event::Backspace => if focused {
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
            Event::Delete => if focused {
                let mut text = self.text.borrow_mut();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i < text.len() {
                    text.remove(text_i);
                    self.text_i.set(min(text_i, text.len()));
                }
            },
            Event::Home => if focused {
                let text = self.text.borrow();
                let mut text_i = self.text_i.get();
                while text.is_char_boundary(text_i) && text_i > 0 {
                    let range = text.char_range_at_reverse(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                }
                self.text_i.set(text_i);
            },
            Event::End => if focused {
                let text = self.text.borrow();
                let mut text_i = self.text_i.get();
                while text.is_char_boundary(text_i) && text_i < text.len() {
                    let range = text.char_range_at(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                }
                self.text_i.set(text_i);
            },
            Event::UpArrow => if focused {
                let text = self.text.borrow();
                let mut text_i = self.text_i.get();

                //Count back to last newline
                let mut offset = 0;
                while text.is_char_boundary(text_i) && text_i > 0 {
                    let range = text.char_range_at_reverse(text_i);
                    text_i = range.next;
                    if range.ch == '\n' {
                        break;
                    }
                    offset += 1;
                }

                //Go to newline before last newline
                while text.is_char_boundary(text_i) && text_i > 0 {
                    let range = text.char_range_at_reverse(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                }

                //Add back offset
                while text.is_char_boundary(text_i) && offset > 0 && text_i < text.len() {
                    let range = text.char_range_at(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                    offset -= 1;
                }

                self.text_i.set(text_i);
            },
            Event::DownArrow => if focused {
                let text = self.text.borrow();
                let mut text_i = self.text_i.get();

                //Count back to last newline
                let mut offset = 0;
                while text.is_char_boundary(text_i) && text_i > 0 {
                    let range = text.char_range_at_reverse(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                    offset += 1;
                }

                //Go to next newline
                while text.is_char_boundary(text_i) && text_i < text.len() {
                    let range = text.char_range_at(text_i);
                    text_i = range.next;
                    if range.ch == '\n' {
                        break;
                    }
                }

                //Add back offset
                while text.is_char_boundary(text_i) && offset > 0 && text_i < text.len() {
                    let range = text.char_range_at(text_i);
                    if range.ch == '\n' {
                        break;
                    }
                    text_i = range.next;
                    offset -= 1;
                }

                self.text_i.set(text_i);
            },
            Event::LeftArrow => if focused {
                let text = self.text.borrow();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i > 0 {
                    self.text_i.set(min(text.char_range_at_reverse(text_i).next, text.len()));
                }
            },
            Event::RightArrow => if focused {
                let text = self.text.borrow();
                let text_i = self.text_i.get();
                if text.is_char_boundary(text_i) && text_i < text.len() {
                    self.text_i.set(min(text.char_range_at(text_i).next, text.len()));
                }
            },
            _ => ()
        }

        self.focused.set(focused);

        focused
    }
}
