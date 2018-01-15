use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::cmp::{max, min};
use std::ops::Deref;
use std::sync::Arc;
use orbclient;

use cell::{CheckSet, CloneCell};
use draw::draw_box;
use event::Event;
use point::Point;
use rect::Rect;
use theme::{Selector, Theme};
use traits::{Click, Enter, EventFilter, Place, Style, Text};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

/// Find next character index
fn next_i(text: &str, text_i: usize) -> usize {
    let slice = &text[text_i..];
    slice
        .char_indices()
        .skip(1)
        .next()
        .unwrap_or((slice.len(), '\0'))
        .0 + text_i
}

/// Find last character index
fn prev_i(text: &str, text_i: usize) -> usize {
    let slice = &text[..text_i];
    slice.char_indices().rev().next().unwrap_or((0, '\0')).0
}

pub struct TextBox {
    pub rect: Cell<Rect>,
    children: RefCell<Vec<Arc<Widget>>>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    pub selector: CloneCell<Selector>,
    pub text: CloneCell<String>,
    pub text_i: Cell<usize>,
    pub text_offset: Cell<Point>,
    pub scroll_offset: Cell<(i32, i32)>,
    pub mask_char: Cell<Option<char>>,
    pub grab_focus: Cell<bool>,
    pub click_callback: RefCell<Option<Arc<Fn(&TextBox, Point)>>>,
    pub enter_callback: RefCell<Option<Arc<Fn(&TextBox)>>>,
    /// If event_filter is defined, all of the events will go trough it
    /// Instead of the default behavior. This allows defining fields that
    /// ex. will only accept numbers and ignore all else, or add some
    /// special behavior for some keys.
    ///
    /// The closure should return None if the event was manually handled,
    /// or should return the event it received if it wants the default
    /// handler deal with it.
    pub event_filter:
        RefCell<Option<Arc<Fn(&TextBox, Event, &mut bool, &mut bool) -> Option<Event>>>>,
    pressed: Cell<bool>,
}

impl TextBox {
    pub fn new() -> Arc<Self> {
        Arc::new(TextBox {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            children: RefCell::new(vec![]),
            selector: CloneCell::new(Selector::new(Some("text-box"))),
            text: CloneCell::new(String::new()),
            text_i: Cell::new(0),
            text_offset: Cell::new(Point::default()),
            scroll_offset: Cell::new((0, 0)),
            mask_char: Cell::new(None),
            grab_focus: Cell::new(false),
            click_callback: RefCell::new(None),
            enter_callback: RefCell::new(None),
            event_filter: RefCell::new(None),
            pressed: Cell::new(false),
        })
    }

    pub fn grab_focus(&self, grab_focus: bool) -> &Self {
        self.grab_focus.set(grab_focus);
        self
    }

    pub fn mask_char(&self, mask_char: Option<char>) -> &Self {
        self.mask_char.set(mask_char);
        self
    }
}

impl Click for TextBox {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Enter for TextBox {
    fn emit_enter(&self) {
        if let Some(ref enter_callback) = *self.enter_callback.borrow() {
            enter_callback(self)
        }
    }

    fn on_enter<T: Fn(&Self) + 'static>(&self, func: T) -> &Self {
        *self.enter_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl EventFilter for TextBox {
    fn handle_event(&self, event: Event, focused: &mut bool, redraw: &mut bool) -> Option<Event> {
        if let Some(ref event_filter) = *self.event_filter.borrow() {
            event_filter(self, event, focused, redraw)
        } else {
            Some(event)
        }
    }

    fn event_filter<T: Fn(&Self, Event, &mut bool, &mut bool) -> Option<Event> + 'static>(
        &self,
        func: T,
    ) -> &Self {
        *self.event_filter.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for TextBox {}

impl Text for TextBox {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        let text = text.into();
        self.text_i.set(text.len());
        self.text.set(text);
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Style for TextBox {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for TextBox {
    fn name(&self) -> &str {
        "TextBox"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn draw(&self, renderer: &mut Renderer, focused: bool, theme: &Theme) {
        let rect = self.rect.get();

        let mut selector = self.selector.get();

        if focused {
            selector = selector.with_pseudo_class("focus");
        }

        draw_box(renderer, rect, theme, &selector);

        let text_i = self.text_i.get();
        let text = self.text.borrow();

        let text_offset = self.text_offset.get();
        let scroll_offset = self.scroll_offset.get();

        let mut x = text_offset.x - scroll_offset.0 * 8;
        let mut y = text_offset.y - scroll_offset.1 * 16;
        let start_x = x;
        for (i, c) in text.char_indices() {
            let mut c_r = Rect::new(x + rect.x, y + rect.y, 8, 16);
            if c == '\n' {
                if focused && i == text_i && rect.contains_rect(&c_r) {
                    //TODO: set this selector as the child of self.selector
                    draw_box(
                        renderer,
                        Rect::new(x + rect.x, y + rect.y, 8, 16),
                        theme,
                        &"selection".into(),
                    );
                }

                x = start_x;
                y += c_r.height as i32;
            } else if c == '\t' {
                c_r.width = 8 * 4;

                if focused && i == text_i && rect.contains_rect(&c_r) {
                    //TODO: set this selector as the child of self.selector
                    draw_box(
                        renderer,
                        Rect::new(x + rect.x, y + rect.y, 8 * 4, 16),
                        theme,
                        &"selection".into(),
                    );
                }

                x += c_r.width as i32;
            } else {
                if rect.contains_rect(&c_r) {
                    if i == text_i && focused {
                        //TODO: set this selector as the child of self.selector
                        draw_box(
                            renderer,
                            Rect::new(x + rect.x, y + rect.y, 8, 16),
                            theme,
                            &"selection".into(),
                        );
                    }
                    if let Some(mask_c) = self.mask_char.get() {
                        renderer.char(
                            x + rect.x,
                            y + rect.y,
                            mask_c,
                            theme.color("color", &selector),
                        );
                    } else {
                        renderer.char(x + rect.x, y + rect.y, c, theme.color("color", &selector));
                    }
                }

                x += c_r.width as i32;
            }
        }

        let c_r = Rect::new(x + rect.x, y + rect.y, 8, 16);
        if focused && text.len() == text_i && rect.contains_rect(&c_r) {
            //TODO: set this selector as the child of self.selector
            draw_box(
                renderer,
                Rect::new(x + rect.x, y + rect.y, 8, 16),
                theme,
                &"selection".into(),
            );
        }
    }

    fn event(&self, event: Event, mut focused: bool, redraw: &mut bool) -> bool {
        // If the event wasn't handled by the custom handler.
        if let Some(event) = self.handle_event(event, &mut focused, redraw) {
            let mut new_text_i = None;
            match event {
                Event::Mouse {
                    point, left_button, ..
                } => {
                    let mut click = false;

                    let rect = self.rect.get();
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
                        if !left_button {
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

                            let text_offset = self.text_offset.get();
                            let scroll_offset = self.scroll_offset.get();

                            let mut x = text_offset.x - scroll_offset.0 * 8;
                            let mut y = text_offset.y - scroll_offset.1 * 16;
                            let start_x = x;
                            for (i, c) in text.char_indices() {
                                let mut c_r = Rect::new(x, y, 8, 16);
                                if c == '\n' {
                                    if click_point.x >= x && click_point.y >= y
                                        && click_point.y < (y + c_r.height as i32)
                                    {
                                        new_text_i = Some(i);
                                        break;
                                    }

                                    x = start_x;
                                    y += c_r.height as i32;
                                } else if c == '\t' {
                                    c_r.width = 8 * 4;

                                    if c_r.contains(click_point) {
                                        new_text_i = Some(i);
                                        break;
                                    }

                                    x += c_r.width as i32;
                                } else {
                                    if c_r.contains(click_point) {
                                        new_text_i = Some(i);
                                        break;
                                    }

                                    x += c_r.width as i32;
                                }
                            }

                            let c_r = Rect::new(x, y, 8, 16);
                            if (new_text_i.is_none() && click_point.x >= x && click_point.y >= y)
                                || click_point.y >= (y + c_r.height as i32)
                            {
                                new_text_i = Some(text.len());
                            }
                        }

                        self.emit_click(click_point);
                    }
                }
                Event::Scroll { y, .. } => {
                    let lines = self.text.borrow().lines().count() as i32;
                    let rows = (self.rect.get().height as i32 - self.text_offset.get().y) / 16;

                    let mut scroll_offset = self.scroll_offset.get();
                    scroll_offset.1 = max(0, min(lines - rows, scroll_offset.1 - y * 3));
                    self.scroll_offset.set(scroll_offset);

                    *redraw = true;
                }
                Event::KeyPressed(key_event) => match key_event.scancode {
                    orbclient::K_ENTER => if focused {
                        if self.enter_callback.borrow().is_some() {
                            self.emit_enter();
                            *redraw = true;
                        } else {
                            let mut text = self.text.borrow_mut();
                            let text_i = self.text_i.get();
                            text.insert(text_i, '\n');
                            new_text_i = Some(next_i(text.deref(), text_i));
                        }
                    },
                    orbclient::K_BKSP => if focused {
                        let mut text = self.text.borrow_mut();
                        let mut text_i = self.text_i.get();

                        if text_i > 0 {
                            text_i = prev_i(text.deref(), text_i);
                            if text_i < text.len() {
                                text.remove(text_i);
                                new_text_i = Some(min(text_i, text.len()));
                            }
                        }
                    },
                    orbclient::K_DEL => if focused {
                        let mut text = self.text.borrow_mut();
                        let text_i = self.text_i.get();

                        if text_i < text.len() {
                            text.remove(text_i);
                            new_text_i = Some(min(text_i, text.len()));
                        }
                    },
                    orbclient::K_HOME => if focused {
                        let text = self.text.borrow();
                        let mut text_i = self.text_i.get();

                        while text_i > 0 {
                            if text[..text_i].chars().rev().next() == Some('\n') {
                                break;
                            }
                            text_i = prev_i(text.deref(), text_i);
                        }

                        new_text_i = Some(text_i);
                    },
                    orbclient::K_END => if focused {
                        let text = self.text.borrow();
                        let mut text_i = self.text_i.get();

                        while text_i < text.len() {
                            if text[text_i..].chars().next() == Some('\n') {
                                break;
                            }
                            text_i = next_i(text.deref(), text_i);
                        }

                        new_text_i = Some(text_i);
                    },
                    orbclient::K_UP => {
                        if focused {
                            let text = self.text.borrow();
                            let mut text_i = self.text_i.get();

                            // Count back to last newline
                            let mut offset = 0;
                            while text_i > 0 {
                                let c = text[..text_i].chars().rev().next();
                                text_i = prev_i(text.deref(), text_i);
                                if c == Some('\n') {
                                    break;
                                }
                                offset += 1;
                            }

                            // Go to newline before last newline
                            while text_i > 0 {
                                if text[..text_i].chars().rev().next() == Some('\n') {
                                    break;
                                }
                                text_i = prev_i(text.deref(), text_i);
                            }

                            // Add back offset
                            while offset > 0 && text_i < text.len() {
                                if text[text_i..].chars().next() == Some('\n') {
                                    break;
                                }
                                text_i = next_i(text.deref(), text_i);
                                offset -= 1;
                            }

                            new_text_i = Some(text_i);
                        }
                    }
                    orbclient::K_DOWN => {
                        if focused {
                            let text = self.text.borrow();
                            let mut text_i = self.text_i.get();

                            // Count back to last newline
                            let mut offset = 0;
                            while text_i > 0 {
                                if text[..text_i].chars().rev().next() == Some('\n') {
                                    break;
                                }
                                text_i = prev_i(text.deref(), text_i);
                                offset += 1;
                            }

                            // Go to next newline
                            while text_i < text.len() {
                                let c = text[text_i..].chars().next();
                                text_i = next_i(text.deref(), text_i);
                                if c == Some('\n') {
                                    break;
                                }
                            }

                            // Add back offset
                            while offset > 0 && text_i < text.len() {
                                if text[text_i..].chars().next() == Some('\n') {
                                    break;
                                }
                                text_i = next_i(text.deref(), text_i);
                                offset -= 1;
                            }

                            new_text_i = Some(text_i);
                        }
                    }
                    orbclient::K_LEFT => if focused {
                        let text = self.text.borrow();
                        let text_i = self.text_i.get();

                        if text_i > 0 {
                            new_text_i = Some(prev_i(text.deref(), text_i));
                        }
                    },
                    orbclient::K_RIGHT => if focused {
                        let text = self.text.borrow();
                        let text_i = self.text_i.get();
                        if text_i < text.len() {
                            new_text_i = Some(next_i(text.deref(), text_i));
                        }
                    },
                    _ => if focused {
                        if let Some(character) = key_event.character {
                            let mut text = self.text.borrow_mut();
                            let text_i = self.text_i.get();
                            text.insert(text_i, character);
                            new_text_i = Some(next_i(text.deref(), text_i));
                        }
                    },
                },
                _ => (),
            }

            if let Some(text_i) = new_text_i {
                self.text_i.set(text_i);
                *redraw = true;

                let text = self.text.borrow();

                let text_offset = self.text_offset.get();
                let mut scroll_offset = self.scroll_offset.get();

                let mut col = 0;
                let mut row = 0;
                for (i, c) in text.char_indices() {
                    if c == '\n' {
                        if i == text_i {
                            break;
                        }

                        col = 0;
                        row += 1;
                    } else if c == '\t' {
                        if i == text_i {
                            break;
                        }

                        col += 4;
                    } else {
                        if i == text_i {
                            break;
                        }

                        col += 1;
                    }
                }

                let rect = self.rect.get();
                let cols = (rect.width as i32 - text_offset.x) / 8;
                let rows = (rect.height as i32 - text_offset.y) / 16;

                if col < scroll_offset.0 {
                    scroll_offset.0 = col;
                }
                if col >= scroll_offset.0 + cols {
                    scroll_offset.0 = col + 1 - cols;
                }
                if row < scroll_offset.1 {
                    scroll_offset.1 = row;
                }
                if row >= scroll_offset.1 + rows {
                    scroll_offset.1 = row + 1 - rows;
                }

                self.scroll_offset.set(scroll_offset);
            }

            if self.grab_focus.check_set(false) {
                focused = true;
                *redraw = true;
            }
        }
        focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
