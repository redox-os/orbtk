use std::sync::Arc;
use std::cell::RefCell;

use widgets::{Content, Widget, WidgetType};
use drawable::RectangleDrawable;
use tree::Node;

pub struct Center {
    child: RefCell<Option<Arc<Widget>>>,
}

impl Center {
    pub fn new() -> Arc<Self> {
        Arc::new(Center {
            child: RefCell::new(None),
        })
    }

    pub fn child<W>(&self, widget: &Arc<W>) -> &Self
    where
        W: Widget,
    {
        *self.child.borrow_mut() = Some(widget.clone());
        self
    }
}

impl Widget for Center {
    fn types(&self) -> Vec<WidgetType> {
        vec![
            WidgetType::SingleChildLayout(Arc::new(|_child: &Arc<Node>| {})),
            WidgetType::Drawable(RectangleDrawable::new()),
        ]
    }

    fn build(&self) -> Content {
        if let Some(ref child) = *self.child.borrow_mut() {
            return Content::Single(child.clone());
        }

        Content::Zero
    }

    fn element(&self) -> &str {
        "center"
    }
}