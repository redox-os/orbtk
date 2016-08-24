use super::{CloneCell, Color, Event, Placeable, Point, Rect, RectExt, Renderer, Widget, WidgetCore};
use super::callback::{Click, Enter};
use super::cell::CheckSet;

use std::cell::Cell;
use std::cmp::min;
use std::ops::Deref;
use std::sync::Arc;

/// Find next character index
fn next_i(text: &str, text_i: usize) -> usize {
    let slice = &text[text_i..];
    slice.char_indices().skip(1).next().unwrap_or((slice.len(), '\0')).0 + text_i
}

/// Find last character index
fn prev_i(text: &str, text_i: usize) -> usize {
    let slice = &text[.. text_i];
    slice.char_indices().rev().next().unwrap_or((0, '\0')).0
}

pub struct TextBox {
    pub core: WidgetCore,
    pub text: CloneCell<String>,
    pub text_i: Cell<usize>,
    pub fg_cursor: Color,
    click_callback: Option<Arc<Fn(&TextBox, Point)>>,
    enter_callback: Option<Arc<Fn(&TextBox)>>,
    pressed: Cell<bool>,
}

impl TextBox {
    pub fn new() -> Self {
        TextBox {
            core: WidgetCore::new(Color::white(), Color::black()),
            text: CloneCell::new(String::new()),
            text_i: Cell::new(0),
            fg_cursor: Color::gray(),
            click_callback: None,
            enter_callback: None,
            pressed: Cell::new(false),
        }
    }

    pub fn text<S: Into<String>>(self, text: S) -> Self {
        let text = text.into();
        self.text_i.set(text.len());
        self.text.set(text);
        self
    }
}

impl Click for TextBox {
    fn emit_click(&self, point: Point) {
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
    fn emit_enter(&self) {
        if let Some(ref enter_callback) = self.enter_callback {
            enter_callback(self)
        }
    }

    fn on_enter<T: Fn(&Self) + 'static>(mut self, func: T) -> Self {
        self.enter_callback = Some(Arc::new(func));

        self
    }
}

impl RectExt for TextBox {
    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }
}

impl Placeable for TextBox {}

impl Widget for TextBox {
    fn draw(&self, renderer: &mut Renderer, focused: bool) {
        let rect = self.core.rect.get();

        renderer.rect(rect, self.core.bg);

        let text_i = self.text_i.get();
        let text = self.text.borrow();

        let mut x = 0;
        let mut y = 0;
        for (i, c) in text.char_indices() {
            if c == '\n' {
                if i == text_i && focused && x + 8 <= rect.width as i32 &&
                   y + 16 <= rect.height as i32 {
                    renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
                }

                x = 0;
                y += 16;
            } else if c == '\t' {
                if x + 8 * 4 <= rect.width as i32 && y + 16 <= rect.height as i32 {
                    if i == text_i && focused {
                        renderer.rect(Rect::new(x + rect.x, y + rect.y, 8 * 4, 16), self.fg_cursor);
                    }
                }
                x += 8 * 4;
            } else {
                if x + 8 <= rect.width as i32 && y + 16 <= rect.height as i32 {
                    if i == text_i && focused {
                        renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
                    }
                    renderer.char(Point::new(x, y) + rect.point(), c, self.core.fg);
                }

                x += 8;
            }
        }

        if text.len() == text_i && focused && x + 8 <= rect.width as i32 &&
           y + 16 <= rect.height as i32 {
            renderer.rect(Rect::new(x + rect.x, y + rect.y, 8, 16), self.fg_cursor);
        }
    }

    fn event(&self, event: Event, mut focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.core.rect.get();
                if rect.contains(point) {
                    if left_button {
                        if self.pressed.check_set(true) {
                            *redraw = true;
                        }
                    } else {
                        if self.pressed.check_set(false) {
                            click = true;
                            *redraw = true;
                        }
                    }
                } else {
                    if ! left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
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
                                if x + 8 <= rect.width as i32 && click_point.x >= x &&
                                   y + 16 <= rect.height as i32 &&
                                   click_point.y >= y &&
                                   click_point.y < y + 16 {
                                    new_text_i = Some(i);
                                    break;
                                }
                                x = 0;
                                y += 16;
                            } else if c == '\t' {
                                if x + 8 * 4 <= rect.width as i32 && click_point.x >= x &&
                                   click_point.x < x + 8 * 4 &&
                                   y + 16 <= rect.height as i32 &&
                                   click_point.y >= y &&
                                   click_point.y < y + 16 {
                                    new_text_i = Some(i);
                                    break;
                                }
                                x += 8 * 4;
                            } else {
                                if x + 8 <= rect.width as i32 && click_point.x >= x &&
                                   click_point.x < x + 8 &&
                                   y + 16 <= rect.height as i32 &&
                                   click_point.y >= y &&
                                   click_point.y < y + 16 {
                                    new_text_i = Some(i);
                                    break;
                                }
                                x += 8;
                            }
                        }

                        if new_text_i.is_none() && x + 8 <= rect.width as i32 &&
                           click_point.x >= x &&
                           y + 16 <= rect.height as i32 &&
                           click_point.y >= y ||
                           click_point.y >= y + 16 {
                            new_text_i = Some(text.len());
                        }

                        if let Some(text_i) = new_text_i {
                            self.text_i.set(text_i);
                        }
                    }

                    self.emit_click(click_point);
                }
            }
            Event::Text { c } => {
                if focused {
                    let mut text = self.text.borrow_mut();
                    let text_i = self.text_i.get();
                    text.insert(text_i, c);
                    self.text_i.set(next_i(text.deref(), text_i));
                    *redraw = true;
                }
            }
            Event::Enter => {
                if focused {
                    if self.enter_callback.is_some() {
                        self.emit_enter();
                    } else {
                        let mut text = self.text.borrow_mut();
                        let text_i = self.text_i.get();
                        text.insert(text_i, '\n');
                        self.text_i.set(next_i(text.deref(), text_i));
                    }
                    *redraw = true;
                }
            }
            Event::Backspace => {
                if focused {
                    let mut text = self.text.borrow_mut();
                    let mut text_i = self.text_i.get();
                    if text_i > 0 {
                        text_i = prev_i(text.deref(), text_i);
                        if text_i < text.len() {
                            text.remove(text_i);
                            self.text_i.set(min(text_i, text.len()));
                        }
                    }
                    *redraw = true;
                }
            }
            Event::Delete => {
                if focused {
                    let mut text = self.text.borrow_mut();
                    let text_i = self.text_i.get();
                    if text_i < text.len() {
                        text.remove(text_i);
                        self.text_i.set(min(text_i, text.len()));
                    }
                    *redraw = true;
                }
            }
            Event::Home => {
                if focused {
                    let text = self.text.borrow();
                    let mut text_i = self.text_i.get();
                    while text_i > 0 {
                        if text[.. text_i].chars().rev().next() == Some('\n') {
                            break;
                        }
                        text_i = prev_i(text.deref(), text_i);
                    }
                    self.text_i.set(text_i);
                    *redraw = true;
                }
            }
            Event::End => {
                if focused {
                    let text = self.text.borrow();
                    let mut text_i = self.text_i.get();
                    while text_i < text.len() {
                        if text[text_i ..].chars().next() == Some('\n') {
                            break;
                        }
                        text_i = next_i(text.deref(), text_i);
                    }
                    self.text_i.set(text_i);
                    *redraw = true;
                }
            }
            Event::UpArrow => {
                if focused {
                    let text = self.text.borrow();
                    let mut text_i = self.text_i.get();

                    // Count back to last newline
                    let mut offset = 0;
                    while text_i > 0 {
                        let c = text[.. text_i].chars().rev().next();
                        text_i = prev_i(text.deref(), text_i);
                        if c == Some('\n') {
                            break;
                        }
                        offset += 1;
                    }

                    // Go to newline before last newline
                    while text_i > 0 {
                        if text[.. text_i].chars().rev().next() == Some('\n') {
                            break;
                        }
                        text_i = prev_i(text.deref(), text_i);
                    }

                    // Add back offset
                    while offset > 0 && text_i < text.len() {
                        if text[text_i ..].chars().next() == Some('\n') {
                            break;
                        }
                        text_i = next_i(text.deref(), text_i);
                        offset -= 1;
                    }

                    self.text_i.set(text_i);
                    *redraw = true;
                }
            }
            Event::DownArrow => {
                if focused {
                    let text = self.text.borrow();
                    let mut text_i = self.text_i.get();

                    // Count back to last newline
                    let mut offset = 0;
                    while text_i > 0 {
                        if text[.. text_i].chars().rev().next() == Some('\n') {
                            break;
                        }
                        text_i = prev_i(text.deref(), text_i);
                        offset += 1;
                    }

                    // Go to next newline
                    while text_i < text.len() {
                        let c = text[text_i ..].chars().next();
                        text_i = next_i(text.deref(), text_i);
                        if c == Some('\n') {
                            break;
                        }
                    }

                    // Add back offset
                    while offset > 0 && text_i < text.len() {
                        if text[text_i ..].chars().next() == Some('\n') {
                            break;
                        }
                        text_i = next_i(text.deref(), text_i);
                        offset -= 1;
                    }

                    self.text_i.set(text_i);
                    *redraw = true;
                }
            }
            Event::LeftArrow => {
                if focused {
                    let text = self.text.borrow();
                    let text_i = self.text_i.get();
                    if text_i > 0 {
                        self.text_i.set(prev_i(text.deref(), text_i));
                    }
                    *redraw = true;
                }
            }
            Event::RightArrow => {
                if focused {
                    let text = self.text.borrow();
                    let text_i = self.text_i.get();
                    if text_i < text.len() {
                        self.text_i.set(next_i(text.deref(), text_i));
                    }
                    *redraw = true;
                }
            }
            _ => (),
        }

        focused
    }
}
