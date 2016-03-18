extern crate orbclient;

use orbclient::BmpFile;

use super::{CloneCell, Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;

pub struct Menu {
    rect: Rect,
    fg: Color,
    actions: Vec<Action>,
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

impl Menu {
    pub fn new() -> Self {
        Menu {
            rect: Rect::default(),
            fg: Color::rgb(0, 0, 0),
            actions: Vec::with_capacity(1),
        }
    }

    pub fn add_action(&mut self, name: &str, icon: Option<BmpFile>) {
        self.actions.insert(Action::new(name), icon);
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
