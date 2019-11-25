use std::{cell::RefCell, collections::BTreeMap, rc::Rc, any::Any};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::WindowShell, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub registry: Rc<RefCell<Registry>>,
}

impl System<Tree, StringComponentStore> for PostLayoutStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        if !self.shell.borrow().update() || !self.shell.borrow().running() {
            return;
        }

        let root = ecm.entity_store().root;

        let window_shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let render_objects = &self.render_objects;
        let layouts = &mut self.layouts.borrow_mut();
        let handlers = &mut self.handlers.borrow_mut();
        let new_states = &mut BTreeMap::new();

        let mut ctx = Context::new(
            (root, ecm),
            window_shell,
            &theme,
            render_objects,
            layouts,
            handlers,
            &self.states,
            new_states
        );

        for (node, state) in &*self.states.borrow() {
            ctx.entity = *node;

            state.update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);

            // Handle messages.
            {
                // todo fix messages.
                // for (entity, messages) in ctx.messages().iter() {
                //     if let Some(state) = self.states.borrow().get(&entity) {
                //         ctx.entity = *entity;
                //         state.receive_messages(&mut ctx, &messages);
                //     }
                // }
            }
        }
    }
}
