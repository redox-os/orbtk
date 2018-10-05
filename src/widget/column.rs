use theme::Selector;
use std::rc::Rc;
use {Alignment, ComponentBox, FlexLayoutObject, LayoutObject, Property, Template, Widget};

pub struct Column {
    pub children: Vec<Rc<Widget>>,
    pub class: String,
}

impl Default for Column {
    fn default() -> Column {
        Column {
            children: vec![],
            class: String::from("column"),
        }
    }
}

impl Widget for Column {
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
        Box::new(FlexLayoutObject::new(Alignment::Vertical))
    }
}
