use super::{WindowBuilder, Window, Rect};

use std::sync::Arc;
use std::any::Any;
use std::cell::RefCell;

pub trait Node: Any {}

pub trait Widget {
    fn build(&self) -> Arc<Node>;
    fn update(&self);
}

impl Node for Widget {}

pub struct Center {
    child: RefCell<Option<Arc<Node>>>,
}

impl Center {
    pub fn new() -> Arc<Self> {
        Arc::new(Center {
            child: RefCell::new(None),
        })
    }

    pub fn child<N>(&self, node: &Arc<N>) -> &Self
        where
            N: Node,
    {
        *self.child.borrow_mut() = Some(node.clone());
        self
    }
}

impl Node for Center {}

pub struct Container {
    child: RefCell<Option<Arc<Node>>>,
}

impl Container {
    pub fn new() -> Arc<Self> {
        Arc::new(Container {
            child: RefCell::new(None),
        })
    }

    pub fn child<N>(&self, node: &Arc<N>) -> &Self
        where
            N: Node,
    {
        *self.child.borrow_mut() = Some(node.clone());
        self
    }
}

impl Node for Container {}

pub struct Text {}

impl Text {
    pub fn new() -> Arc<Self> {
        Arc::new(Text {})
    }
}

impl Node for Text {}



pub struct Button {

}

impl Widget for Button {
    fn build(&self) -> Arc<Node> {
        let center = Center::new();
        center.child(&Text::new());
        center
    }

    fn update(&self) {}
}

pub struct Application {
    window: Window
}

impl Application {
    pub fn new(rect: Rect, title: &str) -> Application {
        Application {
            window: WindowBuilder::new(rect, title).build(),
        }
    }

    pub fn root(&self, _root: &Widget) -> &Self {
        self
    }

    pub fn run(&mut self) {
        self.window.exec();
    }
}

