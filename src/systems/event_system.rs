use std::cell::RefCell;
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use event::{EventStrategy, Handler};
use tree::Tree;
use widget::WidgetContainer;

pub struct EventSystem {
    pub backend: Rc<RefCell<Backend>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Rc<Handler>>>>,
}

impl System<Tree> for EventSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let event_context = backend.event_context();
        let mut target_node = None;

        let mut offsets = BTreeMap::new();
        offsets.insert(tree.root, (0, 0));

        for event_box in event_context.event_queue.borrow_mut().into_iter() {
            for (node, handler) in &*self.handlers.borrow() {
                let mut widget = WidgetContainer::new(*node, ecm, tree);

                let handles_event = handler.handles_event(&event_box, &widget);

                if handles_event {
                    if event_box.strategy == EventStrategy::TopDown {
                        target_node = Some(*node);
                    } else {
                        // bottom up
                        if handler.handle_event(&event_box, &mut widget) {
                            break;
                        }
                    }
                }
            }

            // top down
            if let Some(target_node) = target_node {
                let mut target_node = target_node;
                loop {
                    let mut widget = WidgetContainer::new(target_node, ecm, tree);
                    let entity_has_handler = self.handlers.borrow_mut().contains_key(&target_node);

                    if entity_has_handler
                        && self.handlers.borrow_mut()[&target_node]
                            .handle_event(&event_box, &mut widget)
                    {
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
