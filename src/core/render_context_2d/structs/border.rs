use super::{Brush, Thickness};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Border {
    pub brush: Brush,
    pub thickness: Thickness,
}