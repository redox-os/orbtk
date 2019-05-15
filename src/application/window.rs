use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::prelude::*;
use crate::backend::*;

/// Represents a window. Each window has its own tree, event pipeline and backend.
pub struct WindowShell {
    pub backend_runner: Box<dyn BackendRunner>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub resizable: bool,
}

impl WindowShell {
    /// Executes the given window until quit is requested.
    pub fn run(&mut self) {
        self.backend_runner
            .run(self.update.clone(), self.running.clone());
    }
}
