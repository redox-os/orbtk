/// Internal helper state to handle current glutin window state.
#[derive(Copy, Clone, Default, Debug)]
pub struct WindowState {
    pub size: (usize, usize),
    pub active: bool,
}
