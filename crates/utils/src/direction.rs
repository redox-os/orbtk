#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    ToTop,
    ToTopRight,
    ToRight,
    ToBottomRight,
    ToBottom,
    ToBottomLeft,
    ToLeft,
    ToTopLeft,
}
