/// `FontIcon` represents a font based icon.
#[derive(Default, Clone)]
pub struct FontIcon(pub String);

property!(FontIcon, FontIconProperty, font_icon, font_icon_prop);

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

/// `PrimaryFontIcon` represents a primary font based icon.
#[derive(Default, Clone)]
pub struct PrimaryFontIcon(pub String);

impl From<&str> for PrimaryFontIcon {
    fn from(s: &str) -> PrimaryFontIcon {
        PrimaryFontIcon(s.to_string())
    }
}

impl From<String> for PrimaryFontIcon {
    fn from(s: String) -> PrimaryFontIcon {
        PrimaryFontIcon(s)
    }
}

/// `SecondaryFontIcon` represents a primary font based icon.
#[derive(Default, Clone)]
pub struct SecondaryFontIcon(pub String);

impl From<&str> for SecondaryFontIcon {
    fn from(s: &str) -> SecondaryFontIcon {
        SecondaryFontIcon(s.to_string())
    }
}

impl From<String> for SecondaryFontIcon {
    fn from(s: String) -> SecondaryFontIcon {
        SecondaryFontIcon(s)
    }
}
