use std::sync::Arc;
use super::{Widget, WidgetType};
use drawable::TextDrawable;

pub struct Text {}

impl Text {
    pub fn new() -> Arc<Self> {
        Arc::new(Text {})
    }
}

impl Widget for Text {
    fn types(&self) -> Vec<WidgetType> {
        vec![WidgetType::Drawable(TextDrawable::new("Test"))]
    }
    fn element(&self) -> &str {
        "text"
    }
}