use std::sync::Arc;
use std::cell::RefCell;

use widgets::{Content, Widget, WidgetType};
use tree::{Constraint};

pub struct Row {
    children: RefCell<Vec<Arc<Widget>>>,
}

impl Row {
    pub fn new() -> Arc<Self> {
        Arc::new(Row {
            children: RefCell::new(vec![]),
        })
    }

    pub fn push<W>(&self, widget: &Arc<W>) -> &Self
    where
        W: Widget,
    {
        self.children.borrow_mut().push(widget.clone());
        self
    }
}

impl Widget for Row {
    fn types(&self) -> Vec<WidgetType> {
        vec![
            WidgetType::Layout(Arc::new(|_owener: &Arc<Widget>, _constraint: &Option<Constraint>| -> Constraint {
                Constraint::default()
            })),
        ]
    }

    fn build(&self) -> Content {
        if self.children.borrow().len() > 0 {
            Content::Multi(self.children.borrow_mut().clone())
        } else {
            Content::None
        }
    }

    fn element(&self) -> &str {
        "row"
    }
}
