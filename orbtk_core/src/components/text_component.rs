#[derive(Clone, Default, Debug)]
pub struct TextComponent {
    pub text: String,
}

impl TextComponent {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl From<String> for TextComponent {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl From<&str> for TextComponent {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
