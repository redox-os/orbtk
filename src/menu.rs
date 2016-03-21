extern crate orbclient;

use orbclient::BmpFile;

use super::{CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;

pub struct Menu {
    rect: CopyCell<Rect>,
    text: CloneCell<String>,
    fg: Color,
    entries: Vec<Box<Entry>>,
    click_callback: Option<Arc<Fn(&Menu, Point)>>,
    activated: CopyCell<bool>,
}

pub struct Action {
    rect: CopyCell<Rect>,
    text: CloneCell<String>,
    icon: Option<BmpFile>,
    bg_up: Color,
    bg_down: Color,
    click_callback: Option<Arc<Fn(&Action, Point)>>,
    pressed: CopyCell<bool>,
}

pub struct Separator;

pub trait Entry {
    fn text<S: AsRef<str>>(&mut self) -> S;
}

impl Menu {
    pub fn new(name: &str) -> Self {
        Menu {
            rect: Rect::default(),
            text: CloneCell::new(name.to_owned()),
            fg: Color::rgb(0, 0, 0),
            entries: Vec::with_capacity(10),
            click_callback: None,
            activated: false,
        }
    }

    pub fn add_entry<E: Entry + Place>(mut self, mut entry: E) -> Self {
        {
            let entry_text_len = entry.text().len();
            if self.rect.width < entry_text_len {
                // TODO: consider the icon width and some padding
                self.rect.width = entry_text_len;
            }
        }

        if entry.text().is_empty() {
            // Separator entry goes here
            self.entries.insert(entry.size(self.rect.width, 10));
            self.rect.height += 10;
        } else {
            self.entries.insert(entry.size(self.rect.width, 30));
            self.rect.height += 30;
        }

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
    fn rect(&self) -> &CopyCell<Rect> {
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
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if left_button && self.activated.check_set(true) {
                        *redraw = true;
                    } else if self.pressed.check_set(false) {
                        click = true;
                        *redraw = true;
                    }
                }
            }
        }
    }
}

impl Action {
    pub fn new(text: &str) -> Self {
        Action {
            rect: Rect::default(),
            text: CloneCell::new(name.to_owned()),
            icon: None,
            bg_up: Color::rgb(220, 222, 227),
            bg_down: Color::rgb(203, 205, 210),
            click_callback: None,
            pressed: CopyCell::new(false),
        }
    }

    pub fn add_icon(mut self, icon: BmpFile) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl Place for Action {
    fn rect(&self) -> &CopyCell<Rect> {
        &self.rect
    }
}

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
                        *click = true;
                        redraw = true;
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

impl Entry for Menu {
    fn text<S: AsRef<str>>(&mut self) -> S {
        self.text.get()
    }
}

impl Entry for Action {
    fn text<S: AsRef<str>>(&mut self) -> S {
        self.text.get()
    }
}

impl Entry for Separator {
    fn text<S: AsRef<str>>(&mut self) -> S {
        ""
    }
}
