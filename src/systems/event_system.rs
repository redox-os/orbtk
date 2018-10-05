use std::cell::RefCell;
use std::sync::Arc;

use dces::{EntityComponentManager, System};

use {Backend, Tree};

pub struct EventSystem {
    pub _backend: Arc<RefCell<Backend>>,
}

impl System<Tree> for EventSystem {
    fn run(&self, _tree: &Tree, _ecm: &mut EntityComponentManager) {
       
        
    }
}
