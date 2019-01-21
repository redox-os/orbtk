#[derive(Copy, Clone, PartialEq)]
pub enum Placement {
    Left,
    Right,
}

impl Default for Placement {
    fn default() -> Self {
        Placement::Left
    }
}
