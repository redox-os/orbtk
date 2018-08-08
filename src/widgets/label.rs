use std::sync::Arc;

use cell::CloneCell;
use super::{Widget, WidgetType};
use drawable::TextDrawable;
use theme::{Selector, Style};


pub struct Label {
    text: String,
    selector: CloneCell<Selector>,
}

impl Label {
    pub fn new<S: Into<String>>(text: S) -> Arc<Self> {
        Arc::new(Label {
            text: text.into(),
            selector: CloneCell::new(Selector::new(Some("container"))),
        })
    }
}

impl Widget for Label {
    fn types(&self) -> Vec<WidgetType> {
        vec![
            WidgetType::Drawable(TextDrawable::new(&self.text, self.selector().get())),
            WidgetType::FixedSized {
                width: self.text.len() as u32 * 8,
                height: 16,
            },
            WidgetType::Styleable(self.selector.get()),
        ]
    }
    fn element(&self) -> &str {
        "text"
    }
}

impl Style for Label {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}
