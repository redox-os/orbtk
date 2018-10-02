use std::sync::Arc;

use theme::Selector;
use {Alignment, ComponentBox, FlexLayoutObject, LayoutObject, Property, Template, Widget};

pub struct Row {
    pub children: Vec<Arc<Widget>>,
    pub class: String,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            children: vec![],
            class: String::from("row"),
        }
    }
}

impl Widget for Row {
    fn template(&self) -> Template {
        if self.children.len() == 0 {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children.get(0).unwrap().clone())
        } else {
            Template::Mutli(self.children.iter().map(|child| child.clone()).collect())
        }
    }

    fn properties(&self) -> Vec<Property> {
        vec![ComponentBox::new(Selector::new(Some(self.class.clone())))]
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject::new(Alignment::Horizontal))
    }
}
