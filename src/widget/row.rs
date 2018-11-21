use std::rc::Rc;

use layout_object::{FlexLayoutObject, LayoutObject};
use enums::Alignment;
use widget::{Template, Widget};

/// This layout widget orders its children horizontal.
pub struct Row {
    pub children: Vec<Rc<Widget>>,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            children: vec![],
        }
    }
}

impl Widget for Row {
    fn template(&self) -> Template {
        print!("Row -> ");
        if self.children.is_empty() {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children[0].clone())
        } else {
            Template::Mutli(self.children.to_vec())
        }
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject::new(Alignment::Horizontal))
    }
}
