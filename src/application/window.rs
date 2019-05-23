use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::prelude::*;

use crate::backend;

/// Represents a window. Each window has its own tree, event pipeline and backend.
#[derive(Default)]
pub struct WindowAdapter {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub event_queue: EventQueue,
    pub messages: BTreeMap<Entity, Vec<MessageBox>>,
}

// todo: remove all render_objects etc. from system, handle all with window adapter
// todo: remove rc refcell..
// impl WindowAdapter {
//     pub fn context_provider(&mut self) -> &mut ContextProvider {
//         &mut self.context_provider
//     }
// }

impl backend::WindowAdapter for WindowAdapter {
    fn update(&mut self) {

    }
}

impl Into<Box<backend::WindowAdapter>> for WindowAdapter {
    fn into(self) -> Box<backend::WindowAdapter> {
        Box::new(self)
    }
}

#[derive(Default, Debug)]
pub struct ContextProvider {
    
}