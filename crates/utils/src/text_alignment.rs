/// Used to align a text.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
    Start,
    End,
}

impl ToString for TextAlignment {
    fn to_string(&self) -> String {
        match self {
            TextAlignment::Left => "left".to_string(),
            TextAlignment::Right => "right".to_string(),
            TextAlignment::Center => "center".to_string(),
            TextAlignment::Start => "start".to_string(),
            TextAlignment::End => "end".to_string(),
        }
    }
}
