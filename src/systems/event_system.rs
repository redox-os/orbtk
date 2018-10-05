use std::cell::RefCell;
use std::rc::Rc;

use dces::{EntityComponentManager, System};

use {Backend, Tree};

pub struct EventSystem {
    pub _backend: Rc<RefCell<Backend>>,
}

impl System<Tree> for EventSystem {
    fn run(&self, _tree: &Tree, _ecm: &mut EntityComponentManager) {
       
        
    }
}
