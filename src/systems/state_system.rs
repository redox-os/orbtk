use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, EntityComponentManager, System};

use application::Tree;
use widget::{State, WidgetContainer};

/// The `StateSystem` calls the update methods of widget states.
pub struct StateSystem {
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
    pub update: Rc<Cell<bool>>,
}

impl System<Tree> for StateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() {
            return;
        }

        for (node, state) in &*self.states.borrow() {
            let mut widget = WidgetContainer::new(*node, ecm, tree);
            state.update(&mut widget);
        }
    }
}
