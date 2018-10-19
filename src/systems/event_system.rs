use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use event::EventStrategy;
use tree::Tree;
use state::State;

pub struct EventSystem {
    pub backend: Rc<RefCell<Backend>>,
    pub states: Rc<RefCell<HashMap<Entity, Rc<State>>>>,
}

impl System<Tree> for EventSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let event_context = backend.event_context();
        let mut target_node = None;

        let mut offsets = HashMap::new();
        offsets.insert(tree.root, (0, 0));

        for event_box in event_context.event_queue.borrow_mut().into_iter() {
            for node in tree.into_iter() {
                let entity_has_state = self.states.borrow().contains_key(&node);

                if entity_has_state {
                    let handles_event =
                        self.states.borrow()[&node].handles_event(&event_box, node, ecm);

                    if handles_event {
                        if event_box.strategy == EventStrategy::TopDown {
                            target_node = Some(node);
                        } else {
                            // bottom up
                            if self.states.borrow_mut()[&node].update(&event_box, node, tree, ecm) {
                                break;
                            }
                        }
                    }
                }
            }

            // top down
            if let Some(target_node) = target_node {
                let mut target_node = target_node;
                loop {
                    let entity_has_state = self.states.borrow_mut().contains_key(&target_node);

                    if entity_has_state && self.states.borrow_mut()[&target_node].update(
                        &event_box,
                        target_node,
                        tree,
                        ecm,
                    ) {
                        break;
                    }

                    if target_node == tree.root {
                        break;
                    }

                    target_node = tree.parent[&target_node];
                }
            }
        }
    }
}
