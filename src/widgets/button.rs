use std::sync::Arc;
use super::{Widget, WidgetType, Content, Text};
use layouts::Center;

pub struct Button {}

impl Button {
    pub fn new() -> Arc<Self> {
        Arc::new(Button {})
    }
}

impl Widget for Button {
    fn types(&self) -> Vec<WidgetType> {
        vec![WidgetType::EventHandler()]
    }
    
    fn build(&self) -> Content {
        let center = Center::new();
        center.child(&Text::new());
        Content::Single(center)
    }

    fn element(&self) -> &str {
        "button"
    }

    // maybe use callbacks instead (see eventhandler)
}