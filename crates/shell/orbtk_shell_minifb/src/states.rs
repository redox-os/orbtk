use derive_more::Constructor;
use orbtk_shell::event::Key;

/// Internal helper state to handle current minifb mouse state.
#[derive(Copy, Clone, Default, Debug)]
pub struct MouseState {
    pub mouse_pos: (f32, f32),
    pub button_left: bool,
    pub button_middle: bool,
    pub button_right: bool,
}

/// Internal helper state to handle current minifb window state.
#[derive(Copy, Clone, Default, Debug)]
pub struct WindowState {
    pub size: (usize, usize),
    pub active: bool,
}

/// Internal helper state to handle current minifb key states.
#[derive(Constructor)]
pub struct KeyState {
    pub minifb_key: minifb::Key,
    pub key: Key,
}
