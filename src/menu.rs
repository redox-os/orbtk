extern crate orbclient;

use self::orbclient::BmpFile;

use super::{CloneCell, Color, Event, Place, Point, Rect, Renderer, Widget};
use super::callback::Click;
use super::cell::CheckSet;

use std::cell::Cell;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Menu {
    rect: Cell<Rect>,
    text: CloneCell<String>,
    fg: Color,
    entries: Vec<Box<Entry>>,
    click_callback: Option<Arc<Fn(&Menu, Point)>>,
    activated: Cell<bool>,
}

#[allow(dead_code)]
pub struct Action {
    rect: Cell<Rect>,
    text: CloneCell<String>,
    icon: Option<BmpFile>,
    bg_up: Color,
    bg_down: Color,
    click_callback: Option<Arc<Fn(&Action, Point)>>,
    pressed: Cell<bool>,
}

pub struct Separator;

pub trait Entry {
    fn text(&mut self) -> String;
}

impl Menu {
    pub fn new(name: &str) -> Self {
        Menu {
            rect: Cell::new(Rect::default()),
            text: CloneCell::new(name.to_owned()),
            fg: Color::rgb(0, 0, 0),
            entries: Vec::with_capacity(10),
            click_callback: None,
            activated: Cell::new(false),
        }
    }

    pub fn add_entry<E: 'static + Entry + Place>(mut self, mut entry: E) -> Self {
        let mut rect = self.rect.get();
        let entry_text = entry.text();
        if rect.width < entry_text.len() as u32 {
            // TODO: consider the icon width and some padding
            rect.width = entry_text.len() as u32;
        }

        if entry_text.is_empty() {
            // Separator entry goes here
            entry = entry.size(rect.width, 10);
            rect.height += 10;
        } else {
            entry = entry.size(rect.width, 30);
            rect.height += 30;
        }
        self.entries.push(Box::new(entry));

        self.rect.set(rect);
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
        &self.rect
    }
}

impl Widget for Menu {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        if self.activated.get() {
            renderer.rect(rect, self.fg);
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                //let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if left_button && self.activated.check_set(true) {
                        *redraw = true;
                    } else if self.activated.check_set(false) {
                        //click = true;
                        *redraw = true;
                    }
                }
            }
            _ => (),
        }
        focused
    }
}

impl Action {
    pub fn new(text: &str) -> Self {
        Action {
            rect: Cell::new(Rect::default()),
            text: CloneCell::new(text.to_owned()),
            icon: None,
            bg_up: Color::rgb(220, 222, 227),
            bg_down: Color::rgb(203, 205, 210),
            click_callback: None,
            pressed: Cell::new(false),
        }
    }

    pub fn add_icon(mut self, icon: BmpFile) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl Place for Action {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }
}

/*
impl Widget for Action {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        if self.pressed.get() {
            renderer.rect(rect, self.bg_down);
        } else {
            renderer.rect(rect, self.bg_up);
        }

        //let text = self.text.borrow();

        //let mut x = 0;
        //let mut y = 0;
        //for c in text.chars() {
        //    if c == '\n' {
        //        x = 0;
        //        y += 16;
        //    } else {
        //        if x + 8 <= rect.width as i32 && y + 16 <= rect.height as i32 {
        //            renderer.char(Point::new(x, y) + rect.point(), c, self.fg);
        //        }
        //        x += 8;
        //    }
        //}
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;
                let rect = self.rect.get();

                if rect.contains(point) {
                    if left_button && self.pressed.check_set(true) {
                        *redraw = true;
                    } else if self.pressed.check_set(false) {
                        click = true;
                        *redraw = true;
                    }
                } else if !left_button && self.pressed.check_set(false) {
                    *redraw = true;
                }

                if click {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
        }
    }
}
*/

impl Entry for Menu {
    fn text(&mut self) -> String {
        self.text.get()
    }
}

impl Entry for Action {
    fn text(&mut self) -> String {
        self.text.get()
    }
}

impl Entry for Separator {
    fn text(&mut self) -> String {
        String::new()
    }
}
