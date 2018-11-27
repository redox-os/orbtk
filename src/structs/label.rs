/// The `Label` struct represents a string used for text drawing.
#[derive(Default, Clone)]
pub struct Label(pub String);

impl From<&str> for Label {
    fn from(s: &str) -> Label {
        Label(s.to_string())
    }
}

impl From<String> for Label {
    fn from(s: String) -> Label {
        Label(s)
    }
}