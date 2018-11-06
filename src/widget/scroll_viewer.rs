use std::rc::Rc;

use layout_object::{LayoutObject, ScrollLayoutObject};
use widget::{Property, PropertyResult, Template, Widget};

#[derive(Clone)]
pub struct VerticalOffset(pub i32);

#[derive(Clone)]
pub struct HorizontalOffset(pub i32);

pub struct ScrollViewer {
    pub child: Option<Rc<Widget>>,
    pub vertical_offset: Property<VerticalOffset>,
    pub horizontal_offset: Property<HorizontalOffset>,
}

impl Default for ScrollViewer {
    fn default() -> ScrollViewer {
        ScrollViewer {
            child: None,
            vertical_offset: Property::new(VerticalOffset(0)),
            horizontal_offset: Property::new(HorizontalOffset(0)),
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

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(ScrollLayoutObject)
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.vertical_offset.build(), self.horizontal_offset.build()]
    }
}
