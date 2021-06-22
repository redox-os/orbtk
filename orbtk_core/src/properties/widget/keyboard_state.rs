use std::collections::HashMap;

use crate::shell::Key;

/// Contains the state information for the keyboard.
///
/// The keyboard state tracks which keys are currently pressed. The
/// active state is stored in a lazy-loaded HashMap.
///
/// Beside common key activities, you may need to react on generic
/// modifier keys (`Alt`, `Ctrl`, `Hyper`, `Shift`). Helper functions
/// offer several convenience methods to handle such keyboard events.
/// A generic method comes in handy, if you don't care which
/// modifier key is down (`Shift-left` or `Shift-right` => `Shift`).
/// The example section will also tackle the case, where a combined
/// event (`Ctrl+S`) keyboard state is handled.
///
/// # Example
/// Handle a key press inside a given view.
///
/// ```
/// use orbtk {
///     widgets::behaviors::MouseBehavior,
///     prelude::*;
///     shell::event::{Key, KeyEvent};
/// };
///
/// // [KeyboardState]
///
/// #[derive(Clone)]
/// enum KeyboardAction {
///     Key(KeyEvent),
///     RequestFocus,
/// }
///
/// #[derive(AsAny, Default)]
/// struct KeyboardState {
///     action: Option<KeyboardAction>,
///     event_adapter: EventAdapter,
/// }
///
/// // Implementation of KeyboardActions for trait `State`
/// impl State for KeyboardState {
///     fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
///         if let Some(action) = self.action.clone() {
///             match action {
///                 KeyboardAction::Key(key_event) => {
///                     match key_event.key {
///                         Key::Control => {
///                             // Ctrl+'a'
///                             if key_event.Key::A(false) {
///                                 self.handle_control_a(ctx);
///                             }
///                         }
///                         Key::Down => {
///                             self.handle_down_key(ctx);
///                         }
///                         Key::Enter => {
///                             self.handle_enter_key(ctx);
///                         }
///                         Key::Left => {
///                             self.handle_left_key(ctx);
///                         }
///                         Key::Right => {
///                             self.handle_right_key(ctx);
///                         }
///                         Key::Up => {
///                             self.handle_up_key(ctx);
///                         }
///                         _ => {}
///                     }
///                 }
///                 KeyboardAction::RequestFocus => {
///                     self.request_focus(ctx);
///                 }
///             }
///             self.action = None;
///         }
///
/// // associated functions
/// impl KeyboardState {
///     fn action(&mut self, action: DirectoryListAction) {
///         self.action = Some(action);
/// }
///
/// fn handle_down_key(&mut self, ctx: &mut Context<'_>) {
///     /// here goes your rust code to act on `Key::Down`
///     println!("Handle: `Key::Down`")
///     }
/// }
///
/// fn handle_enter_key(&mut self, ctx: &mut Context<'_>) {
///     /// here goes your rust code to act on `Key::Enter`
///     println!("Handle: `Key::Enter`")
///     }
/// }
///
/// fn handle_left_key(&mut self, ctx: &mut Context<'_>) {
///     /// here goes your rust code to act on `Key::Left`
///     println!("Handle: `Key::Left`")
///     }
/// }
///
/// fn handle_right_key(&mut self, ctx: &mut Context<'_>) {
///     /// here goes your rust code to act on `Key::Right`
///     println!("Handle: `Key::Right`")
///     }
/// }
///
/// fn handle_up_key(&mut self, ctx: &mut Context<'_>) {
///     /// here goes your rust code to act on `Key::Up`
///     println!("Handle: `Key::Up`")
///     }
/// }
///
/// // [KeyboardView]
/// // ... your view code
///
/// ```
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
        assert_eq!(state.is_key_down(Key::ShiftL), false);
        // Set the state down and immediately check
        state.set_key_state(Key::ShiftL, true);
        assert_eq!(state.is_key_down(Key::ShiftL), true);
        state.set_key_state(Key::ShiftL, false);
        assert_eq!(state.is_key_down(Key::ShiftL), false);
        // Set quit a few in a row
        state.set_key_state(Key::ShiftL, true);
        state.set_key_state(Key::ShiftR, true);
        state.set_key_state(Key::Space, true);
        state.set_key_state(Key::Control, true);
        state.set_key_state(Key::Alt, true);
        // Ensure each of these are still down
        assert_eq!(state.is_key_down(Key::ShiftL), true);
        assert_eq!(state.is_key_down(Key::ShiftR), true);
        assert_eq!(state.is_key_down(Key::Space), true);
        assert_eq!(state.is_key_down(Key::Control), true);
        assert_eq!(state.is_key_down(Key::Alt), true);
    }

    #[test]
    /// Test for the convenience methods
    fn test_convenience() {
        let mut state = KeyboardState::default();
        // Check to ensure they are all false
        assert_eq!(state.is_alt_down(), false);
        assert_eq!(state.is_ctrl_down(), false);
        assert_eq!(state.is_shift_down(), false);
        // Set ctrl and alt to true and check
        state.set_key_state(Key::Control, true);
        assert_eq!(state.is_ctrl_down(), true);

        state.set_key_state(Key::Alt, true);
        assert_eq!(state.is_alt_down(), true);

        assert_eq!(state.is_shift_down(), false);
        // Set shift (via L)
        state.set_key_state(Key::ShiftL, true);
        assert_eq!(state.is_shift_down(), true);
        // Set shift (via R and L both set)
        state.set_key_state(Key::ShiftR, true);
        assert_eq!(state.is_shift_down(), true);
        // Disable L Shift to ensure correct result with just R held
        state.set_key_state(Key::ShiftL, false);
        assert_eq!(state.is_shift_down(), true);
        // Disable both shift keys
        state.set_key_state(Key::ShiftR, false);
        assert_eq!(state.is_shift_down(), false);
        // Disable alt and ctrl and check again
        state.set_key_state(Key::Control, false);
        assert_eq!(state.is_ctrl_down(), false);

        state.set_key_state(Key::Alt, false);
        assert_eq!(state.is_alt_down(), false);
    }
}
