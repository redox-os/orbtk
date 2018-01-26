use std::cell::Cell;

use super::Point;

use orbclient;

pub use self::event_manager::*;

mod event_manager;

pub trait Handleable {
    fn handled(&self) -> &Cell<bool>;
}

/// A key event (such as a pressed key)
#[derive(Debug)]
pub struct KeyEventArgs {
    /// The charecter of the key
    pub character: Option<char>,
    /// The scancode of the key
    pub scancode: u8,
    /// Describes if the event is handled
    handled: Cell<bool>,
}

impl Clone for KeyEventArgs {
    fn clone(&self) -> Self {
        KeyEventArgs {
            character: self.character,
            scancode: self.scancode,
            handled: Cell::new(self.handled.get())
        }
     }
}

impl KeyEventArgs {
    pub fn from_orbital_key_event(key_event: orbclient::KeyEvent) -> Self {
        let character = {
            if key_event.character != '\0' && key_event.character != '\x1B'
                && key_event.character != '\n'
            {
                Some(key_event.character)
            } else {
                None
            }
        };

        let scancode = {
            if key_event.character == '\n' {
                orbclient::K_ENTER
            } else {
                key_event.scancode
            }
        };

        KeyEventArgs {
            character,
            scancode,
            handled: Cell::new(false),
        }
    }
}

impl Handleable for KeyEventArgs {
    fn handled(&self) -> &Cell<bool> {
        &self.handled
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub struct MouseEventArgs {
    pub point: Point,
    pub button: MouseButton,
    /// Describes if the event is handled
    handled: Cell<bool>,
}

impl MouseEventArgs {
    pub fn new(point: Point, button: MouseButton) -> Self {
        MouseEventArgs {
            point, 
            button,
            handled: Cell::new(false),
        }
    }
}

impl Clone for MouseEventArgs {
    fn clone(&self) -> Self {
        MouseEventArgs {
            point: self.point,
            button: self.button,
            handled: Cell::new(self.handled.get())
        }
     }
}

impl Handleable for MouseEventArgs {
    fn handled(&self) -> &Cell<bool> {
        &self.handled
    }
}

#[derive(Debug)]
pub struct MouseMoveEventArgs {
    pub point: Point,
    /// Describes if the event is handled
    handled: Cell<bool>,
}

impl Clone for MouseMoveEventArgs {
    fn clone(&self) -> Self {
        MouseMoveEventArgs {
            point: self.point,
            handled: Cell::new(self.handled.get())
        }
     }
}

impl MouseMoveEventArgs {
    pub fn new(point: Point) -> Self {
        MouseMoveEventArgs {
            point,
            handled: Cell::new(false),
        }
    }
}

impl Handleable for MouseMoveEventArgs {
    fn handled(&self) -> &Cell<bool> {
        &self.handled
    }
}

#[derive(Debug)]
pub struct ScrollEventArgs {
    pub point: Point,
    /// Describes if the event is handled
    handled: Cell<bool>,
}

impl Clone for ScrollEventArgs {
    fn clone(&self) -> Self {
        ScrollEventArgs {
            point: self.point,
            handled: Cell::new(self.handled.get())
        }
     }
}

impl ScrollEventArgs {
    pub fn new(point: Point) -> Self {
        ScrollEventArgs {
            point,
            handled: Cell::new(false),
        }
    }
}

impl Handleable for ScrollEventArgs {
    fn handled(&self) -> &Cell<bool> {
        &self.handled
    }
}


#[derive(Clone, Debug)]
pub enum Event {
    KeyDownEvent(KeyEventArgs),
    KeyUpEvent(KeyEventArgs),
    MouseDownEvent(MouseEventArgs),
    MouseUpEvent(MouseEventArgs),
    MouseMoveEvent(MouseMoveEventArgs),
    ScrollEvent(ScrollEventArgs),

    // outdated events
    Init,

    Resize {
        width: u32,
        height: u32,
    },

    Unknown,

    Mouse {
        point: Point,
        left_button: bool,
        middle_button: bool,
        right_button: bool,
    },

    Key,

    KeyPressed(KeyEventArgs),

    KeyReleased(KeyEventArgs),

    Scroll {
        x: i32,
        y: i32,
    },  
}
