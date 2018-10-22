use std::rc::Rc;

use layout_object::{LayoutObject, PaddingLayoutObject};
use render_object::{RectangleRenderObject, RenderObject};
use theme::Selector;
use widget::{Property, PropertyResult, Template, Widget};

pub struct Container {
    pub child: Option<Rc<Widget>>,
    pub selector: Property<Selector>,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            child: None,
            selector: Property::new(Selector::new(Some(String::from("container")))),
        }
    }
}

impl Widget for Container {
    fn template(&self) -> Template {
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.selector.build()]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(PaddingLayoutObject)
    }
}
