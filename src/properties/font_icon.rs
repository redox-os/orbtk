/// `FontIcon` represents a font based icon.
#[derive(Default, Clone)]
pub struct FontIcon(pub String);

impl From<&str> for FontIcon {
    fn from(s: &str) -> FontIcon {
        FontIcon(s.to_string())
    }
}

impl From<String> for FontIcon {
    fn from(s: String) -> FontIcon {
        FontIcon(s)
    }
}