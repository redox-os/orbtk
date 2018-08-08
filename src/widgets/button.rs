use std::sync::Arc;
use super::{Content, Label, Widget, WidgetType};
use cell::CloneCell;
use layouts::{Container, Center};
use theme::{Selector, Style};

pub struct Button {
    selector: CloneCell<Selector>,
    text: String,
}

impl Button {
    pub fn new<S: Into<String>>(text: S) -> Arc<Self> {
        Arc::new(Button {
            selector: CloneCell::new(Selector::new(Some("button"))),
            text: text.into(),
        })
    }
}

impl Widget for Button {
    fn types(&self) -> Vec<WidgetType> {
        vec![
            WidgetType::EventHandler(),
        ]
    }

    fn build(&self) -> Content {
        /* todo: content! marco
            content!(
                Container (
                    Center (
                        Label (
                            "Button"
                        )
                    )
                )
            )
        */
        let container = Container::new();
        container.selector().set(self.selector().get());
        let center = Center::new();
        let label = Label::new(&self.text[..]);
        label.selector().set(self.selector().get());
        center.child(&label);
        container.child(&center);
        Content::Single(container)
    }

    fn element(&self) -> &str {
        "button"
    }

    // maybe use callbacks instead (see eventhandler)
}

impl Style for Button {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}
