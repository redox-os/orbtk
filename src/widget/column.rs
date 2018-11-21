use std::rc::Rc;

use layout_object::{FlexLayoutObject, LayoutObject};
use enums::Alignment;
use widget::{Template, Widget};

/// This layout widget orders its children vertical.
pub struct Column {
    pub children: Vec<Rc<Widget>>,
}

impl Default for Column {
    fn default() -> Column {
        Column {
            children: vec![],
        }
    }
}

impl Widget for Column {
    fn template(&self) -> Template {
        print!("Column -> ");
        if self.children.is_empty() {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children[0].clone())
        } else {
            Template::Mutli(self.children.to_vec())
        }
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject::new(Alignment::Vertical))
    }
}
