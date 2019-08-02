use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, ecm: &mut EntityComponentManager<Tree>, root: Entity) {
        // Add css id to global id map.
        let id = if let Ok(selector) = ecm.component_store().borrow_component::<Selector>(node) {
            if let Some(id) = &selector.0.id {
                Some((node, id.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = ecm
                .component_store_mut()
                .borrow_mut_component::<Global>(root)
            {
                global.id_map.insert(id, entity);
            }
        }
    }

    // Read all initial data from css
    fn read_init_from_theme(&self, context: &mut Context) {
        context.update_theme_properties();
    }
}

impl System<Tree> for InitSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        let root = ecm.entity_store().root;

        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;;

        if debug {
            crate::shell::log("\n------ Widget tree ------\n".to_string());

            print_tree(root, 0, ecm);

            crate::shell::log("\n------ Widget tree ------\n".to_string());
        }

        let window_shell = &mut self.shell.borrow_mut();

        // init css ids
        let root = ecm.entity_store().root;
        for node in ecm.entity_store().clone().into_iter() {
            self.init_id(node, ecm, root);

            let mut context = Context::new(node, ecm, window_shell, &theme);

            if let Some(state) = self.states.borrow().get(&node) {
                state.init(&mut context);
            }

            self.read_init_from_theme(&mut context);
        }
    }
}

fn print_tree(entity: Entity, depth: usize, ecm: &mut EntityComponentManager<Tree>) {
    let name = Name::get(entity, ecm);
    let selector = Selector::get(entity, ecm);

    crate::shell::log(format!(
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
