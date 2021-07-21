use std::collections::HashMap;

use crate::shell::Key;

/// Contains the state information for the keyboard.
///
/// This currently tracks which keys are currently pressed.
///
/// The key state is stored in a lazy-loaded HashMap.
///
/// There are several convenience methods to check common modifiers (ctrl, shift, alt, etc).
/// This is useful if you don't care which shift key is down.
#[derive(Default, Clone, Debug, PartialEq)]
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

    /// Returns whether or not any home key is down.
    pub fn is_home_down(&self) -> bool {
        self.is_key_down(Key::Home)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// A quick test to ensure that the items are properly set.
    fn basic_test() {
        let mut state = KeyboardState::default();
        // Do a quick check beforehand
        assert!(!state.is_key_down(Key::ShiftL));
        // Set the state down and immediately check
        state.set_key_state(Key::ShiftL, true);
        assert!(state.is_key_down(Key::ShiftL));
        state.set_key_state(Key::ShiftL, false);
        assert!(!state.is_key_down(Key::ShiftL));
        // Set quit a few in a row
        state.set_key_state(Key::ShiftL, true);
        state.set_key_state(Key::ShiftR, true);
        state.set_key_state(Key::Space, true);
        state.set_key_state(Key::Control, true);
        state.set_key_state(Key::Alt, true);
        // Ensure each of these are still down
        assert!(state.is_key_down(Key::ShiftL));
        assert!(state.is_key_down(Key::ShiftR));
        assert!(state.is_key_down(Key::Space));
        assert!(state.is_key_down(Key::Control));
        assert!(state.is_key_down(Key::Alt));
    }

    #[test]
    /// Test for the convenience methods
    fn test_convenience() {
        let mut state = KeyboardState::default();
        // Check to ensure they are all false
        assert!(!state.is_alt_down());
        assert!(!state.is_ctrl_down());
        assert!(!state.is_shift_down());
        // Set ctrl and alt to true and check
        state.set_key_state(Key::Control, true);
        assert!(state.is_ctrl_down());

        state.set_key_state(Key::Alt, true);
        assert!(state.is_alt_down());

        assert!(!state.is_shift_down());
        // Set shift (via L)
        state.set_key_state(Key::ShiftL, true);
        assert!(state.is_shift_down());
        // Set shift (via R and L both set)
        state.set_key_state(Key::ShiftR, true);
        assert!(state.is_shift_down(), "{}", true);
        // Disable L Shift to ensure correct result with just R held
        state.set_key_state(Key::ShiftL, false);
        assert!(state.is_shift_down());
        // Disable both shift keys
        state.set_key_state(Key::ShiftR, false);
        assert!(!state.is_shift_down());
        // Disable alt and ctrl and check again
        state.set_key_state(Key::Control, false);
        assert!(!state.is_ctrl_down());

        state.set_key_state(Key::Alt, false);
        assert!(!state.is_alt_down());
    }
}
