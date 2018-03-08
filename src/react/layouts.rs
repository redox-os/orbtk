use std::sync::Arc;
use std::cell::RefCell;

use react::{Content, RectangleDrawable, Widget, WidgetType, Node};

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

// impl Layout for Center {
//    fn child(&self) -> &RefCell<Option<Arc<Widget>>> {
//        &self.child
//    }
// }

pub struct Container {
    child: RefCell<Option<Arc<Widget>>>,
}

impl Container {
    pub fn new() -> Arc<Self> {
        Arc::new(Container {
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

impl Widget for Container {
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
        "container"
    }
}

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
            WidgetType::MultiChildrenLayout(Arc::new(|_children: Vec<&Arc<Node>>| {})),
            WidgetType::Drawable(RectangleDrawable::new()),
        ]
    }

    fn build(&self) -> Content {
        if self.children.borrow().len() > 0 {
            Content::Multi(self.children.borrow_mut().clone())
        } else {
            Content::Zero
        }
    }

    fn element(&self) -> &str {
        "row"
    }
}
