use std::rc::Rc;

use widget::{Template, Widget};

/// Use this layout widget to overlay its children (on z axis).
pub struct Stack {
    pub children: Vec<Rc<Widget>>,
}

impl Default for Stack {
    fn default() -> Stack {
        Stack { children: vec![] }
    }
}

impl Widget for Stack {
    fn template(&self) -> Template {
        print!("Stack -> ");
        if self.children.is_empty() {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children[0].clone())
        } else {
            Template::Mutli(self.children.to_vec())
        }
    }
}
