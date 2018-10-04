
pub enum MouseButton {
    Left,
    Middle,
    Right
}

pub enum MouseEvent {
    Move((i32, i32)),
    Down(MouseButton),
    Up(MouseButton),
}