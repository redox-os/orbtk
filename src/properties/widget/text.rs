use crate::widget::Template;

/// The `Text` struct represents a string used for text drawing.
#[derive(Default, Clone)]
pub struct Text(pub String);

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

pub trait TextProperty: Sized + From<Template> + Into<Template> {
    fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
        Self::from(transform(self.into()))
    }

    fn text<L: Into<Text>>(self, text: L) -> Self {
        self.template(|template| {
            template.property(text.into())
        })
    }
}