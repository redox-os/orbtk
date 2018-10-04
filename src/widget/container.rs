use std::sync::Arc;

use theme::Selector;
use {
    ComponentBox, LayoutObject, PaddingLayoutObject, Property, RectangleRenderObject, RenderObject,
    Template, Widget,
};

pub struct Container {
    pub child: Option<Arc<Widget>>,
    pub class: String,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            child: None,
            class: String::from("container"),
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

    fn properties(&self) -> Vec<Property> {
        vec![
            ComponentBox::new(Selector::new(Some(self.class.clone()))),
        ]
    }

    fn render_object(&self) -> Option<Arc<RenderObject>> {
        Some(Arc::new(RectangleRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(PaddingLayoutObject)
    }
}
