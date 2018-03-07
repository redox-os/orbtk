

use react::{Center, WidgetType, TextDrawable, Widget, Content};

use std::sync::Arc;

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
}

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

    // maybe use callbacks instead (see eventhandler)
}
