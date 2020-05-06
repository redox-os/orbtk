use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use crate::{prelude::*, utils::Point};

#[derive(Default, Clone)]
pub struct ContextProvider {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handler_map: Rc<RefCell<EventHandlerMap>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub event_queue: Rc<RefCell<EventQueue>>,
    pub mouse_position: Rc<Cell<Point>>,
}

impl ContextProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
