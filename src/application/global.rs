use std::collections::HashMap;

use dces::prelude::Entity;

use crate::prelude::*;

#[derive(Default)]
/// The `Global` struct is used to define global `properties` that could be access application width.
pub struct Global {
    /// Contains the current focused widget.
    pub focused_widget: Option<Entity>,

    /// Used to reference widgets by its css id.
    pub id_map: HashMap<String, Entity>,

    /// Stores the state of the keyboard
    pub keyboard_state: KeyboardState,

}

#[derive(Default)]
/// Contains the state information for the keyboard.
///
/// This currently tracks whick keys are currently pressed.
///
/// The key state is stored in a lazy-loaded HashMap.
///
/// There are several convenience methods to check common modifiers (ctrl, shift, alt, etc).
/// This is useful if you don't care which shift key is down.
pub struct KeyboardState {
    key_list: HashMap<Key, bool>,
}

impl KeyboardState {
    /// Sets whether or not the given key is currently pressed
    pub fn set_key_state(&mut self, key: Key, pressed: bool) {
        self.key_list.insert(key, pressed);
    }
    /// Returns whether or not the requested key is pressed
    pub fn is_key_down(&self, key: Key) -> bool {
        match self.key_list.get(&key) {
            // If we have the key on this list, return its state
            Some(item) => *item,
            // Otherwise, it hasn't been set as down
            None => false,
        }
    }
    /// Returns whether or not any shift key is down.
    pub fn is_shift_down(&self) -> bool {
        self.is_key_down(Key::ShiftL) || self.is_key_down(Key::ShiftR)
    }
    /// Returns whether or not any alt key is down.
    pub fn is_alt_down(&self) -> bool {
        self.is_key_down(Key::Alt)
    }
    /// Returns whether or not any control key is down.
    pub fn is_ctrl_down(&self) -> bool {
        self.is_key_down(Key::Control)
    }
}
