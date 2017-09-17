use orbclient::{Color, Renderer};
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::{CloneCell, CheckSet};
use event::Event;
use point::Point;
use rect::Rect;
use theme::{BUTTON_BACKGROUND, BUTTON_BG_SELECTION, BUTTON_FOREGROUND, BUTTON_BORDER,
            ITEM_BACKGROUND, ITEM_FOREGROUND, ITEM_SELECTION};
use traits::{Click, Place, Text};
use widgets::Widget;

pub struct Menu {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
    text: CloneCell<String>,
    bg_pressed: Color,
    fg_border: Color,
    text_offset: Cell<Point>,
    entries: RefCell<Vec<Arc<Entry>>>,
    click_callback: RefCell<Option<Arc<Fn(&Menu, Point)>>>,
    pressed: Cell<bool>,
    activated: Cell<bool>,
}

pub struct Separator {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
}

pub trait Entry: Widget {
    fn entry_text(&self) -> String;
}

impl Menu {
    pub fn new<S: Into<String>>(name: S) -> Arc<Self> {
        Arc::new(Menu {
            rect: Cell::new(Rect::default()),
            bg: BUTTON_BACKGROUND,
            fg: BUTTON_FOREGROUND,
            text: CloneCell::new(name.into()),
            bg_pressed: BUTTON_BG_SELECTION,
            fg_border: BUTTON_BORDER,
            text_offset: Cell::new(Point::default()),
            entries: RefCell::new(Vec::new()),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
            activated: Cell::new(false),
        })
    }

    pub fn add<T: Entry>(&self, new_entry: &Arc<T>) {
        let mut rect = self.rect.get();
        let text_width = new_entry.entry_text().len() as u32 * 8;
        if rect.width < text_width {
            rect.width = text_width;
        }

        let mut y = rect.y + rect.height as i32;
        for entry in self.entries.borrow().iter() {
            let mut entry_rect = entry.rect().get();
            y += entry_rect.height as i32;

            if entry_rect.width < rect.width {
                entry_rect.width = rect.width;
                entry.rect().set(entry_rect);
            } else {
                rect.width = entry_rect.width;
            }
        }
        rect.y = y;
        new_entry.rect().set(rect);
        self.entries.borrow_mut().push(new_entry.clone());
    }
}

impl Text for Menu {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Click for Menu {
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

impl Place for Menu {}

impl Widget for Menu {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        if self.activated.get() {
            renderer.rect(rect.x, rect.y, rect.width, rect.height, self.bg_pressed);
        } else {
            renderer.rect(rect.x, rect.y, rect.width, rect.height, self.bg);
        }

        let text = self.text.borrow();
        let mut point = self.text_offset.get();
        for c in text.chars() {
            if c == '\n' {
                point.x = self.text_offset.get().x;
                point.y += 16;
            } else {
                if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                    renderer.char(point.x + rect.x, point.y + rect.y, c, self.fg);
                }
                point.x += 8;
            }
        }

        renderer.rect(rect.x, rect.y + rect.height as i32 - 1, rect.width, 1, self.fg_border);

        if self.activated.get() {
            let mut max_width = 0;
            let mut max_height = 0;

            for entry in self.entries.borrow().iter() {
                let r = entry.rect().get();
                max_width = ::std::cmp::max(max_width, r.x + r.width as i32 - rect.x);
                max_height = ::std::cmp::max(max_height, r.y + r.height as i32 - rect.y - rect.height as i32);
            }

            renderer.rect(rect.x - 1, rect.y + rect.height as i32 - 1, max_width as u32 + 2, max_height as u32 + 2, self.fg_border);

            for entry in self.entries.borrow().iter() {
                entry.draw(renderer, _focused);
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        let mut ignore_event = false;
        if self.activated.get() {
            for entry in self.entries.borrow().iter() {
                if entry.event(event, focused, redraw) {
                    ignore_event = true;
                    self.pressed.set(true);
                }
            }
        }

        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if left_button {
                        self.pressed.set(!self.pressed.get());

                        if self.activated.check_set(true) {
                            click = true;
                            *redraw = true;
                        }
                    } else {
                        if !self.pressed.get() {
                            if self.activated.check_set(false) {
                                click = true;
                                *redraw = true;
                            }
                        }
                    }
                } else {
                    if !ignore_event {
                        if left_button {
                            self.pressed.set(false);
                        } else {
                            if !self.pressed.get() {
                                if self.activated.check_set(false) {
                                    *redraw = true;
                                }
                            }
                        }
                    }
                }

                if click {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
            _ => (),
        }
        focused
    }
}

pub struct Action {
    rect: Cell<Rect>,
    bg: Color,
    fg: Color,
    text: CloneCell<String>,
    bg_pressed: Color,
    text_offset: Cell<Point>,
    click_callback: RefCell<Option<Arc<Fn(&Action, Point)>>>,
    pressed: Cell<bool>,
    hover: Cell<bool>,
}

impl Action {
    pub fn new<S: Into<String>>(text: S) -> Arc<Self> {
        Arc::new(Action {
            rect: Cell::new(Rect::default()),
            bg: ITEM_BACKGROUND,
            fg: ITEM_FOREGROUND,
            text: CloneCell::new(text.into()),
            bg_pressed: ITEM_SELECTION,
            text_offset: Cell::new(Point::default()),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
            hover: Cell::new(false),
        })
    }
}

impl Click for Action {
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

impl Text for Action {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Widget for Action {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        let (bg, fg) = if self.hover.get() {
            (self.bg_pressed, self.bg)
        } else {
            (self.bg, self.fg)
        };

        renderer.rect(rect.x, rect.y, rect.width, rect.height, bg);

        let text = self.text.borrow();
        let mut point = self.text_offset.get();
        for c in text.chars() {
            if c == '\n' {
                point.x = self.text_offset.get().x;
                point.y += 16;
            } else {
                if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                    renderer.char(point.x + rect.x, point.y + rect.y, c, fg);
                }
                point.x += 8;
            }
        }
    }

    fn event(&self, event: Event, _focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;
                let rect = self.rect.get();

                if rect.contains(point) {
                    if self.hover.check_set(true) {
                        *redraw = true;
                    }

                    if left_button {
                        if self.pressed.check_set(true) {
                            *redraw = true;
                        }
                    } else {
                        if self.pressed.check_set(false) {
                            click = true;
                            self.hover.set(false);
                            *redraw = true;
                        }
                    }
                } else {
                    if self.hover.check_set(false) {
                        *redraw = true;
                    }

                    if !left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
                    }
                }

                if click {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
            _ => (),
        }

        false
    }
}

impl Entry for Action {
    fn entry_text(&self) -> String {
        self.text.get()
    }
}

impl Separator {
    pub fn new() -> Arc<Self> {
        Arc::new(Separator {
            rect: Cell::new(Rect::default()),
            bg: ITEM_BACKGROUND,
            fg: ITEM_FOREGROUND,
        })
    }
}

impl Widget for Separator {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        renderer.rect(rect.x, rect.y, rect.width, rect.height, self.bg);

        let line_y = rect.y + rect.height as i32 / 2;
        renderer.rect(rect.x, line_y, rect.width, 1, self.fg);
    }

    fn event(&self, event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        let mut ignore_event = false;
        match event {
            Event::Mouse { point, .. } => {
                let rect = self.rect.get();
                if rect.contains(point) {
                    ignore_event = true;
                }
            }
            _ => (),
        }
        ignore_event
    }
}

impl Entry for Separator {
    fn entry_text(&self) -> String {
        String::new()
    }
}
