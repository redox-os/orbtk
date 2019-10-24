use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

impl System<Tree, ComponentStore> for PostLayoutStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, ComponentStore>) {
        if !self.update.get() || !self.running.get() {
            return;
        }

        let root = ecm.entity_store().root;

        let window_shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();

        let mut context = Context::new(
            root,
            ecm,
            window_shell,
            &theme,
            self.render_objects.clone(),
            self.layouts.clone(),
            self.handlers.clone(),
            self.states.clone(),
        );

        for (node, state) in &*self.states.borrow() {
            context.entity = *node;

            state.update_post_layout(&mut context);

            // Handle messages.
            {
                // todo fix messages.
                // for (entity, messages) in context.messages().iter() {
                //     if let Some(state) = self.states.borrow().get(&entity) {
                //         context.entity = *entity;
                //         state.receive_messages(&mut context, &messages);
                //     }
                // }
            }
        }
    }
}
