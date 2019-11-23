use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::WindowShell, tree::Tree};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub registry: Rc<RefCell<Registry>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, store: &mut StringComponentStore, root: Entity) {
        // Add css id to global id map.
        let id = if let Ok(selector) = store.get::<Selector>("selector", node) {
            if let Some(id) = &selector.id {
                Some((node, id.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = store.get_mut::<Global>("global", root) {
                global.id_map.insert(id, entity);
            }
        }
    }

    // Read all initial data from css
    fn read_init_from_theme(&self, ctx: &mut Context) {
        ctx.widget().update_theme_by_state(true);
    }
}

impl System<Tree, StringComponentStore> for InitSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        let root = ecm.entity_store().root;

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
        let window_shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut current_node = root;

        loop {
            self.init_id(current_node, ecm.component_store_mut(), root);

            {
                let render_objects = &self.render_objects;
                let layouts = &mut self.layouts.borrow_mut();
                let handlers = &mut self.handlers.borrow_mut();
                let new_states = &mut BTreeMap::new();

                let mut ctx = Context::new(
                    (current_node, ecm),
                    window_shell,
                    &theme,
                    render_objects,
                    layouts,
                    handlers,
                    &self.states,
                    new_states,
                );

                if let Some(state) = self.states.borrow().get(&current_node) {
                    state.init(&mut *self.registry.borrow_mut(), &mut ctx);
                }

                self.read_init_from_theme(&mut ctx);
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
