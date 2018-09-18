use dces::ComponentBox;
use std::sync::Arc;

use theme::Selector;
use {Container, Label, Template, Widget};

pub struct Button;

impl Widget for Button {
    fn template(&self) -> Template {
        let mut container = Container::new();
        container.child(Arc::new(Label::new(Selector::new(Some("button")))));
        Template::Single(Arc::new(container))
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![ComponentBox::new(Selector::new(Some("button")))]
    }
}
