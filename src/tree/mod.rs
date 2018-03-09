use std::sync::Arc;
use std::cell::{Cell, RefCell};

use super::{Rect, Widget};

pub struct Node {
    widget: Arc<Widget>,
    parent: Option<Arc<Node>>,
    children: RefCell<Vec<Arc<Node>>>,
    rect: Cell<Rect>,
    // todo: maybe store also selector
}

impl Node {
    pub fn new_root(widget: &Arc<Widget>) -> Arc<Self> {
        Arc::new(Node {
            widget: widget.clone(),
            parent: None,
            children: RefCell::new(vec![]),
            rect: Default::default(),
        })
    }

    pub fn new(widget: &Arc<Widget>, parent: &Arc<Node>) -> Arc<Self> {
        Arc::new(Node {
            widget: widget.clone(),
            parent: Some(parent.clone()),
            children: RefCell::new(vec![]),
            rect: Default::default(),
        })
    }

    pub fn widget(&self) -> &Arc<Widget> {
        &self.widget
    }

    pub fn parent(&self) -> &Option<Arc<Node>> {
        &self.parent
    }

    pub fn push(&self, child: &Arc<Node>) {
        self.children.borrow_mut().push(child.clone());
    }

    pub fn children(&self) -> &RefCell<Vec<Arc<Node>>> {
        &self.children
    }

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }
}