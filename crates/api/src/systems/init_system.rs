use std::{cell::RefCell, rc::Rc};

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, theming::Selector, tree::Tree};

use std::sync::{Arc, RwLock};

/// This system is used to initializes the widgets.
#[derive(Constructor)]
pub struct InitSystem {
    context_provider: ContextProvider,
    registry: Arc<RwLock<Registry>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, store: &mut StringComponentStore, root: Entity) {
        // Add css id to global id map.
        let id = if let Ok(id) = store.get::<String>("id", node) {
            Some((node, id.clone()))
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = store.get_mut::<Global>("global", root) {
                global.id_map.insert(id, entity);
            }
        }
    }
}

impl System<Tree, StringComponentStore, RenderContext2D> for InitSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        let root = ecm.entity_store().root();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        if debug {
            crate::shell::CONSOLE.log("\n------ Widget tree ------\n".to_string());

            print_tree(root, 0, ecm);

            crate::shell::CONSOLE.log("\n------ Widget tree ------\n".to_string());
        }

        // init css ids
        let theme = ecm
            .component_store()
            .get::<Global>("global", root)
            .unwrap()
            .theme
            .clone();

        let mut current_node = root;

        loop {
            self.init_id(current_node, ecm.component_store_mut(), root);

            {
                let mut ctx = Context::new(
                    (current_node, ecm),
                    &theme,
                    &self.context_provider,
                    render_context,
                );

                if let Some(state) = self
                    .context_provider
                    .states
                    .write()
                    .unwrap()
                    .get_mut(&current_node)
                {
                    state.init(&mut *self.registry.write().unwrap(), &mut ctx);
                }

                drop(ctx);
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

pub fn print_tree(
    entity: Entity,
    depth: usize,
    ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
) {
    let name = ecm.component_store().get::<String>("name", entity).unwrap();

    let selector = if let Ok(selector) = ecm.component_store().get::<Selector>("selector", entity) {
        selector.clone()
    } else {
        Selector::default()
    };

    crate::shell::CONSOLE.log(format!(
        "{}{} (entity: {}{})",
        "| ".repeat(depth),
        name,
        entity.0,
        selector
    ));

    for child in ecm.entity_store().clone().children.get(&entity).unwrap() {
        print_tree(*child, depth + 1, ecm);
    }
}
