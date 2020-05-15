use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, render::RenderContext2D, tree::Tree};

/// Handles the inner cleanup while window is closing.
#[derive(Constructor)]
pub struct CleanupSystem {
    context_provider: ContextProvider,
    registry: Rc<RefCell<Registry>>,
}

impl System<Tree, StringComponentStore, RenderContext2D> for CleanupSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        // let mut shell = self.shell.borrow_mut();
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
                if !self
                    .context_provider
                    .states
                    .borrow()
                    .contains_key(&current_node)
                {
                    skip = true;
                }

                let mut keys = vec![];

                if !skip {
                    let render_objects = &self.context_provider.render_objects;
                    let layouts = &self.context_provider.layouts;
                    let handler_map = &self.context_provider.handler_map;
                    let states = &self.context_provider.states;
                    let event_queue = &self.context_provider.event_queue;
                    let registry = &mut self.registry.borrow_mut();
                    let new_states = &mut BTreeMap::new();

                    let mut ctx = Context::new(
                        (current_node, ecm),
                        &theme,
                        render_objects,
                        layouts,
                        handler_map,
                        states,
                        new_states,
                        event_queue,
                        render_context,
                        &self.context_provider.window_sender,
                        &self.context_provider.shell_sender,
                    );

                    if let Some(state) = self
                        .context_provider
                        .states
                        .borrow_mut()
                        .get_mut(&current_node)
                    {
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
