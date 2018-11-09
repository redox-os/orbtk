use std::rc::Rc;

use layout_object::{CenterLayoutObject, LayoutObject};
use widget::{Template, Widget};

/// This layout widget centers its children within itself.
pub struct Center {
    pub child: Option<Rc<Widget>>,
}

impl Default for Center {
    fn default() -> Center {
        Center {
             child: None,
        }
    }
}

impl Widget for Center {
    fn template(&self) -> Template {
        print!("Center -> ");
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(CenterLayoutObject)
    }
}
