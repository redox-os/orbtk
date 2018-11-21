use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, EntityComponentManager, System};

use state::State;
use tree::Tree;
use widget::WidgetContainer;

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
