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
