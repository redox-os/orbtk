/// Internal helper state to handle current mouse state.
#[derive(Copy, Clone, Default, Debug)]
pub struct MouseState {
    pub button_left: bool,
    pub button_middle: bool,
    pub button_right: bool,
    pub mouse_pos: (f32, f32),
}

/// Internal helper state to handle current window state.
#[derive(Copy, Clone, Default, Debug)]
pub struct WindowState {
    pub active: bool,
    pub size: (usize, usize),
}
