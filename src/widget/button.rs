use theme::Selector;
use std::rc::Rc;
use {Container, Property, Template, TextBlock, Widget};

pub struct Button {
    pub label: String,
    pub class: String,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: String::from("Button"),
            class: String::from("button"),
        }
    }
}

impl Widget for Button {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            class: self.class.clone(),
            child: Some(Rc::new(TextBlock {
                label: self.label.clone(),
                class: self.class.clone(),
            })),
        }))
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property::new(Selector::new(Some(self.class.clone())))]
    }
}
