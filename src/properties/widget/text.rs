/// The `Text` struct represents a string used for text drawing.
#[derive(Default, Clone)]
pub struct Text(pub String);

property!(Text, TextProperty, text, shared_text);

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text(s.to_string())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text(s)
    }
}

// todo tests!!!