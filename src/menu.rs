extern crate orbclient;

use orbclient::BmpFile;

use super::{CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;

pub struct Menu {
    rect: CopyCell<Rect>,
    text: CloneCell<String>,
    fg: Color,
    entries: Vec<Box<Entry>>,
}

pub struct Action {
    rect: CopyCell<Rect>,
    text: CloneCell<String>,
    icon: Option<BmpFile>,
    bg_up: Color,
    bg_down: Color,
    click_callback: Option<Arc<Fn()>>,
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
        }
    }

    pub fn add_entry<E: Entry>(mut self, entry: E) -> Self {
        self.actions.insert(entry);
        self
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

impl Widget for Action {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();

        if self.pressed.get() {
            renderer.rect(rect, self.bg_down);
        } else {
            renderer.rect(rect, self.bg_up);
        }

        let text = self.text.borrow();

        let mut x = 0;
        let mut y = 0;
        for c in text.chars() {
            if c == '\n' {
                x = 0;
                y += 16;
            } else {
                if x + 8 <= rect.width as i32 && y + 16 <= rect.height as i32 {
                    renderer.char(Point::new(x, y) + rect.point(), c, self.fg);
                }
                x += 8;
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
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
        "separator"
    }
}
