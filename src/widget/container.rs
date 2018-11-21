use std::rc::Rc;

use event::{EventHandler};
use layout_object::{LayoutObject, PaddingLayoutObject};
use render_object::{RectangleRenderObject, RenderObject};
use theme::Selector;
use widget::{Property, PropertyResult, Template, Widget};

/// The `Container` layout surrounds its child with a padding. Draws a box arround the child.
pub struct Container {
    pub child: Option<Rc<Widget>>,
    pub selector: Property<Selector>,
    pub event_handlers: Vec<Rc<EventHandler>>,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            child: None,
            selector: Property::new(Selector::new(Some(String::from("container")))),
            event_handlers: vec![],
        }
    }
}

impl Widget for Container {
    fn template(&self) -> Template {
        print!("Container -> ");
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

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        self.event_handlers.to_vec()
    }
}
