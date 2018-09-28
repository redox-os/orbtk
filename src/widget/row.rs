use std::cell::Cell;
use std::sync::Arc;

use theme::Selector;
use {
    Alignment, ComponentBox, FlexLayoutObject, LayoutObject, Property, RectangleRenderObject,
    RenderObject, Template, Widget,
};

pub struct Row {
    pub children: Vec<Arc<Widget>>,
    pub class: String,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            children: vec![],
            class: String::from("container"),
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

    fn properties(&self) -> Vec<Property> {
        vec![ComponentBox::new(Selector::new(Some(self.class.clone())))]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(FlexLayoutObject {
            direction: Alignment::Horizontal,
            ix: Cell::new(0),
            major_per_flex: Cell::new(0),
            minor: Cell::new(0),
        })
    }
}
