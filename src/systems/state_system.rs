use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use dces::{Entity, EntityComponentManager, System};

use state::State;
use tree::Tree;
use widget::WidgetContainer;

pub struct StateSystem {
    pub states: Rc<RefCell<HashMap<Entity, Rc<State>>>>,
}

impl System<Tree> for StateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        for (node, state) in &*self.states.borrow() {
            let mut widget = WidgetContainer::new(*node, ecm, tree);
            state.update(&mut widget);
        }
    }
}
