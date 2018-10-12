use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use dces::{Entity, EntityComponentManager, System};

use {Backend, EventHandler, Tree};

pub struct EventSystem {
    pub backend: Rc<RefCell<Backend>>,
    pub event_handlers: Rc<RefCell<HashMap<TypeId, RefCell<HashMap<Entity, Rc<EventHandler>>>>>>,
}

// todo event stratety bubble, ...
// better splitting of mouse events, ... 
// todo: ouse offset
// todo: mutable handlers

impl System<Tree> for EventSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
       
        let mut backend = self.backend.borrow_mut();
        let event_context = backend.event_context();

        let mut offsets = HashMap::new();
        offsets.insert(tree.root, (0, 0));

        for event_box in event_context.event_queue.borrow_mut().into_iter() {
            for node in tree.into_iter() {
                let event_type = event_box.event_type();

                if self.event_handlers.borrow().contains_key(&event_type) {
                    let handler_map = &self.event_handlers.borrow()[&event_type];

                    if handler_map.borrow().contains_key(&node) {
                        let handler = &handler_map.borrow()[&node];

                        if handler.check_condition(&event_box, node, ecm) {
                            handler.update(node, tree, ecm);
                            handler.emit();
                        }                    
                    }
                }              
            }
        }
    }
}
