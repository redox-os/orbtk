
#[derive(Copy, Clone)]
pub enum Direction {
    None,
    Left,
    Up,
    Right,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}
