use std::sync::Arc;
use std::cell::RefCell;

use react::{WidgetType, RectangleDrawable, Widget, Content};

pub trait Layout {
    fn child(&self) -> &RefCell<Option<Arc<Widget>>>;
}

impl Widget for Layout {
    fn build(&self) -> Content {
        if let Some(ref child) = *self.child().borrow_mut() {
            return Content::Single(child.clone())
        }

        Content::Zero
    }
}

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
            WidgetType::SingleChildLayout(Arc::new(|_child: &Arc<Widget>| {})),
            WidgetType::Drawable(RectangleDrawable::new()),
        ]
    }
}

impl Layout for Center {
   fn child(&self) -> &RefCell<Option<Arc<Widget>>> {
       &self.child
   }
}

pub struct Container {
    child: RefCell<Option<Arc<Widget>>>,
}

impl Container {
    pub fn new() -> Arc<Self> {
        Arc::new(Container {
            child: RefCell::new(None),
        })
    }

    pub fn child<W>(&self, element: &Arc<W>) -> &Self
    where
        W: Widget,
    {
        *self.child.borrow_mut() = Some(element.clone());
        self
    }
}

impl Widget for Container {
    fn types(&self) -> Vec<WidgetType> {
        vec![
            WidgetType::SingleChildLayout(Arc::new(|_child: &Arc<Widget>| {})),
            WidgetType::Drawable(RectangleDrawable::new()),
        ]
    }
}

impl Layout for Container {
   fn child(&self) -> &RefCell<Option<Arc<Widget>>> {
       &self.child
   }
}
