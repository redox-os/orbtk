use std::rc::Rc;

use layout_object::{FlexLayoutObject, LayoutObject};
use enums::Alignment;
use theme::Selector;
use widget::{Property, PropertyResult, Template, Widget};


pub struct Row {
    pub children: Vec<Rc<Widget>>,
    pub selector: Property<Selector>,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            children: vec![],
            selector: Property::new(Selector::new(Some(String::from("column")))),
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

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.selector.build()]
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject::new(Alignment::Horizontal))
    }
}
