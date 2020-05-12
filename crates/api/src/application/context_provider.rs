use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
    sync::mpsc
};

use crate::{prelude::*, utils::Point, shell::WindowRequest};

/// Temporary solution to share dependencies. Will be refactored soon.
#[derive(Clone)]
pub struct ContextProvider {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handler_map: Rc<RefCell<EventHandlerMap>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub event_queue: Rc<RefCell<EventQueue>>,
    pub mouse_position: Rc<Cell<Point>>,
    pub window_sender: mpsc::Sender<WindowRequest>
}

impl ContextProvider {
    /// Creates a new context provider.
    pub fn new(window_sender: mpsc::Sender::<WindowRequest>) -> Self {
       ContextProvider {
           render_objects: Rc::new(RefCell::new(BTreeMap::new())),
           layouts: Rc::new(RefCell::new(BTreeMap::new())),
           handler_map: Rc::new(RefCell::new(EventHandlerMap::new())),
           states: Rc::new(RefCell::new(BTreeMap::new())),
           event_queue: Rc::new(RefCell::new(EventQueue::new())),
           mouse_position: Rc::new(Cell::new(Point::new(0.0, 0.0))),
           window_sender
       }
    }
}
