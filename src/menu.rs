extern crate orbimage;

use self::orbimage::Image;

use super::{CloneCell, Color, Event, Place, Point, Rect, Renderer, Widget, WidgetCore, WidgetPlace};
use super::callback::Click;
use super::cell::CheckSet;

use std::cell::Cell;
use std::sync::Arc;

pub struct Menu {
    pub core: WidgetCore,
    text: CloneCell<String>,
    bg_pressed: Color,
    text_offset: Point,
    entries: Vec<Box<Entry>>,
    click_callback: Option<Arc<Fn(&Menu, Point)>>,
    pressed: Cell<bool>,
    activated: Cell<bool>,
}

pub struct Action {
    core: WidgetCore,
    text: CloneCell<String>,
    icon: Option<Image>,
    bg_pressed: Color,
    text_offset: Point,
    click_callback: Option<Arc<Fn(&Action, Point)>>,
    pressed: Cell<bool>,
    hover: Cell<bool>,
}

pub struct Separator {
    core: WidgetCore,
}

pub trait Entry: Widget {
    fn text(&mut self) -> String;
    fn rect(&self) -> &Cell<Rect>;
}

impl Menu {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Menu {
            core: WidgetCore::new(Color::rgb(220, 222, 227), Color::rgb(0, 0, 0)),
            text: CloneCell::new(name.into()),
            bg_pressed: Color::rgb(203, 205, 210),
            text_offset: Point::default(),
            entries: Vec::with_capacity(10),
            click_callback: None,
            pressed: Cell::new(false),
            activated: Cell::new(false),
        }
    }

    pub fn add_action(&mut self, mut action: Action) {
        let mut action_rect = self.core.rect.get();
        let action_text_width = action.text().len() as u32 * 8;
        if action_rect.width < action_text_width {
            // TODO: consider the icon width and some padding
            action_rect.width = action_text_width;
        }

        let mut y = action_rect.y + action_rect.height as i32;
        for entry in self.entries.iter() {
            let mut entry_rect = entry.rect().get();
            y += entry_rect.height as i32;

            if entry_rect.width < action_rect.width {
                entry_rect.width = action_rect.width;
                entry.rect().set(entry_rect);
            } else {
                action_rect.width = entry_rect.width;
            }
        }
        action_rect.y = y;
        action.rect().set(action_rect);
        self.entries.push(Box::new(action));
    }

    pub fn add_separator(&mut self) {
        let mut sep_rect = self.core.rect.get();

        let mut y = sep_rect.y + sep_rect.height as i32;
        for entry in self.entries.iter() {
            let entry_rect = entry.rect().get();
            y += entry_rect.height as i32;

            if entry_rect.width > sep_rect.width {
                sep_rect.width = entry_rect.width;
            }
        }
        sep_rect.y = y;

        let separator = Separator::new();
        separator.rect().set(sep_rect);
        self.entries.push(Box::new(separator));
    }

    pub fn text<S: Into<String>>(self, text: S) -> Self {
        self.text.set(text.into());
        self
    }

    pub fn text_offset(mut self, x: i32, y: i32) -> Self {
        self.text_offset = Point::new(x, y);
        self
    }
}

impl Click for Menu {
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

impl Place for Menu {
    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }
}

impl WidgetPlace for Menu {}

impl Widget for Menu {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();

        if self.activated.get() {
            renderer.rect(rect, self.bg_pressed);
        } else {
            renderer.rect(rect, self.core.bg);
        }

        let text = self.text.borrow();
        let mut point = self.text_offset;
        for c in text.chars() {
            if c == '\n' {
                point.x = 0;
                point.y += 16;
            } else {
                if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                    renderer.char(point + rect.point(), c, self.core.fg);
                }
                point.x += 8;
            }
        }

        if self.activated.get() {
            for entry in self.entries.iter() {
                entry.draw(renderer, _focused);
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        let mut ignore_event = false;
        if self.activated.get() {
            for entry in self.entries.iter() {
                if entry.event(event, focused, redraw) {
                    ignore_event = true;
                    self.pressed.set(true);
                }
            }
        }

        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.core.rect.get();
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

impl Action {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Action {
            core: WidgetCore::new(Color::rgb(220, 222, 227), Color::rgb(0, 0, 0)),
            text: CloneCell::new(text.into()),
            icon: None,
            bg_pressed: Color::rgb(203, 205, 210),
            text_offset: Point::default(),
            click_callback: None,
            pressed: Cell::new(false),
            hover: Cell::new(false),
        }
    }

    pub fn add_icon(mut self, icon: Image) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text_offset(mut self, x: i32, y: i32) -> Self {
        self.text_offset = Point::new(x, y);
        self
    }
}

impl Click for Action {
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

impl Widget for Action {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();

        if self.hover.get() {
            renderer.rect(rect, self.bg_pressed);
        } else {
            renderer.rect(rect, self.core.bg);
        }

        let text = self.text.borrow();
        let mut point = self.text_offset;
        for c in text.chars() {
            if c == '\n' {
                point.x = 0;
                point.y += 16;
            } else {
                if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                    renderer.char(point + rect.point(), c, self.core.fg);
                }
                point.x += 8;
            }
        }
    }

    fn event(&self, event: Event, _focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;
                let rect = self.core.rect.get();

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
    fn text(&mut self) -> String {
        self.text.get()
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }
}

impl Separator {
    pub fn new() -> Self {
        Separator {
            core: WidgetCore::new(Color::rgb(220, 222, 227), Color::rgb(0, 0, 0)),
        }
    }
}

impl Widget for Separator {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();
        renderer.rect(rect, self.core.bg);

        let line_y = rect.y + rect.height as i32 / 2;
        let start = Point::new(rect.x, line_y);
        let end = Point::new(rect.x + rect.width as i32, line_y);
        renderer.line(start, end, self.core.fg);
    }

    fn event(&self, event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        let mut ignore_event = false;
        match event {
            Event::Mouse { point, .. } => {
                let rect = self.core.rect.get();
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
    fn text(&mut self) -> String {
        String::new()
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }
}
