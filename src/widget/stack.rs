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
        if self.children.len() == 0 {
            Template::Empty
        } else if self.children.len() == 1 {
            Template::Single(self.children.get(0).unwrap().clone())
        } else {
            Template::Mutli(self.children.iter().map(|child| child.clone()).collect())
        }
    }
}
