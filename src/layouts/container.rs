use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::CloneCell;
use drawable::RectangleDrawable;
use structs::Thickness;
use theme::{Selector, Style};
// use tree::{Constraint};
use widgets::{Content, Widget, WidgetType};

pub struct Container {
    child: RefCell<Option<Arc<Widget>>>,
    padding: Cell<Thickness>,
    selector: CloneCell<Selector>,
}

impl Container {
    pub fn new() -> Arc<Self> {
        Arc::new(Container {
            child: RefCell::new(None),
            padding: Cell::new(Thickness::default()),
            selector: CloneCell::new(Selector::new(Some("container"))),
        })
    }

    pub fn child<W>(&self, widget: &Arc<W>) -> &Self
    where
        W: Widget,
    {
        *self.child.borrow_mut() = Some(widget.clone());
        self
    }

    pub fn padding(&self) -> &Cell<Thickness> {
        &self.padding
    }
}

impl Widget for Container {
    fn types(&self) -> Vec<WidgetType> {
        // let padding = self.padding.get();
        vec![
            // WidgetType::Layout(Arc::new(
            //     move |owener: &Arc<Widget>, constraint: &Option<Constraint>| -> Constraint {
            //         let mut rect = owener.render_bounds().get();

            //         if let Some(constraint) = constraint {
            //             let width = {
            //                 if constraint.width > 0 {
            //                     constraint.width
            //                 } else if constraint.max_width > 0 {
            //                     constraint.max_width
            //                 } else {
            //                     constraint.min_width
            //                 }
            //             };

            //             let height = {
            //                 if constraint.height > 0 {
            //                     constraint.height
            //                 } else if constraint.max_height > 0 {
            //                     constraint.max_height
            //                 } else {
            //                     constraint.min_height
            //                 }
            //             };

            //             rect.width = width;
            //             rect.height = height;
            //         }

            //         owener.render_bounds().set(rect);

            //         Constraint {
            //             width: 0,
            //             height: 0,
            //             min_width: 0,
            //             min_height: 0,
            //             max_width: rect.width - padding.left as u32
            //                 - padding.right as u32,
            //             max_height: rect.height - padding.top as u32
            //                 - padding.bottom as u32,
            //         }
            //     },
            // )),
            WidgetType::Drawable(RectangleDrawable::new(self.selector.get())),
            WidgetType::Styleable(self.selector.get()),
        ]
    }

    fn build(&self) -> Content {
        if let Some(ref child) = *self.child.borrow_mut() {
            return Content::Single(child.clone());
        }

        Content::None
    }

    fn element(&self) -> &str {
        "container"
    }
}

impl Style for Container {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}
