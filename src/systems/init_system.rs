use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub backend: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, ecm: &mut EntityComponentManager) {
        // Add css id to global id map.
        let id = if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
            if let Some(id) = &selector.0.id {
                Some((node, id.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = ecm.borrow_mut_component::<Global>(0) {
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
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let theme = ecm.borrow_component::<Theme>(tree.root).unwrap().0.clone();

        #[cfg(feature = "debug")]
            let debug = true;
        #[cfg(not(feature = "debug"))]
            let debug = false;

        if debug {
            println!("\n------ Widget tree ------\n");

            print_tree(tree.root, 0, tree, ecm);

            println!("\n------ Widget tree ------\n");
        }

        let window_shell = &mut self.backend.borrow_mut();

        // init css ids
        for node in tree.into_iter() {
            self.init_id(node, ecm);

            let mut context = Context::new(
                node,
                ecm,
                tree,
                window_shell,
                &theme,
            );

            if let Some(state) = self.states.borrow().get(&node) {
                state.init(&mut context);
            }

            self.read_init_from_theme(&mut context);
        }
    }
}

fn print_tree(entity: Entity, depth: usize, tree: &Tree, ecm: &mut EntityComponentManager) {
    let name = Name::get(entity, ecm);
    let selector = Selector::get(entity, ecm);

    println!("{}{} (entity: {}{})", "| ".repeat(depth), name, entity, selector);

    for child in tree.children.get(&entity).unwrap() {
        print_tree(*child, depth + 1, tree, ecm);
    }
}