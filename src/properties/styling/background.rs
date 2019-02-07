use crate::structs::Brush;

// todo: documentation and methods
pub struct Background {
    pub value: Brush,
}

impl Default for Background {
    fn default() -> Background {
        Background {
            value: Brush::from("#000000"),
        }
    }
}
