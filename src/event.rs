use super::Point;

use orbclient;

/// A key event (such as a pressed key)
#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
    /// The charecter of the key
    pub character: Option<char>,
    /// The scancode of the key
    pub scancode: u8,
}

impl KeyEvent {
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

        KeyEvent {
            character,
            scancode,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Init,

    Mouse {
        point: Point,
        left_button: bool,
        middle_button: bool,
        right_button: bool,
    },

    KeyPressed(KeyEvent),

    KeyReleased(KeyEvent),

    Scroll {
        x: i32,
        y: i32,
    },

    Resize {
        width: u32,
        height: u32,
    },

    Unknown,
}
