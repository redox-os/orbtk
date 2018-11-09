use std::rc::Rc;

use widget::{Property, PropertyResult, Template, Widget};
use layout_object::{LayoutObject, ScrollLayoutObject};

#[derive(Default, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

pub struct ScrollViewer {
    pub child: Option<Rc<Widget>>,
    pub offset: Property<Offset>,
}

impl Default for ScrollViewer {
    fn default() -> ScrollViewer {
        ScrollViewer {
            child: None,
            offset: Property::new(Offset::default()),
        }
    }
}

impl Widget for ScrollViewer {
    fn template(&self) -> Template {
        print!("ScrollViewer -> ");
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.offset.build()]
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(ScrollLayoutObject)
    }
}
