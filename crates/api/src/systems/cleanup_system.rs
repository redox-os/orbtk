use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::Shell, tree::Tree};

/// Handles the inner cleanup while window is closing.
pub struct CleanupSystem {
    pub shell: Rc<RefCell<Shell<ShellAdapter>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub registry: Rc<RefCell<Registry>>,
}

impl System<Tree, StringComponentStore, ContextProvider<'_>> for CleanupSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        let mut shell = self.shell.borrow_mut();
        let root = ecm.entity_store().root();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut current_node = root;

        loop {
            let mut skip = false;

            {
                if !self.states.borrow().contains_key(&current_node) {
                    skip = true;
                }

                let mut keys = vec![];

                if !skip {
                    let render_objects = &self.render_objects;
                    let layouts = &mut self.layouts.borrow_mut();
                    let handlers = &mut self.handlers.borrow_mut();
                    let registry = &mut self.registry.borrow_mut();
                    let new_states = &mut BTreeMap::new();

                    let mut ctx = Context::new(
                        (current_node, ecm),
                        &mut shell,
                        &theme,
                        render_objects,
                        layouts,
                        handlers,
                        &self.states,
                        new_states,
                    );

                    if let Some(state) = self.states.borrow_mut().get_mut(&current_node) {
                        state.cleanup(registry, &mut ctx);
                    }

                    keys.append(&mut ctx.new_states_keys());
                }
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }
    }
}
