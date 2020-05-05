use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::Shell, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub shell: Rc<RefCell<Shell<ShellAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub registry: Rc<RefCell<Registry>>,
}

impl PostLayoutStateSystem {
    fn remove_widget(
        &self,
        entity: Entity,
        theme: &Theme,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        shell: &mut Shell<ShellAdapter>,
    ) {
        {
            let render_objects = &self.render_objects;
            let layouts = &mut self.layouts.borrow_mut();
            let handlers = &mut self.handlers.borrow_mut();
            let registry = &mut self.registry.borrow_mut();
            let new_states = &mut BTreeMap::new();

            let mut ctx = Context::new(
                (entity, ecm),
                shell,
                theme,
                render_objects,
                layouts,
                handlers,
                &self.states,
                new_states,
            );

            if let Some(state) = self.states.borrow_mut().get_mut(&entity) {
                state.cleanup(registry, &mut ctx);
            }

            drop(ctx);
        }
        self.states.borrow_mut().remove(&entity);

        ecm.remove_entity(entity);
        self.layouts.borrow_mut().remove(&entity);
        self.render_objects.borrow_mut().remove(&entity);
        self.handlers.borrow_mut().remove(&entity);
    }
}

impl System<Tree, StringComponentStore, ContextProvider<'_>> for PostLayoutStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        if !self.shell.borrow().update() || !self.shell.borrow().running() {
            return;
        }

        let root = ecm.entity_store().root();

        let mut shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();
        let mut remove_widget_list: Vec<Entity> = vec![];

        {
            let render_objects = &self.render_objects;
            let layouts = &mut self.layouts.borrow_mut();
            let handlers = &mut self.handlers.borrow_mut();
            let new_states = &mut BTreeMap::new();

            let mut ctx = Context::new(
                (root, ecm),
                shell,
                &theme,
                render_objects,
                layouts,
                handlers,
                &self.states,
                new_states,
            );

            for (node, state) in &mut *self.states.borrow_mut() {
                ctx.entity = *node;

                state.update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);
                remove_widget_list.append(ctx.remove_widget_list());

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

        for child in remove_widget_list {
            let mut entities = vec![child];

            for child in &ecm.entity_store().children[&child] {
                entities.push(*child);
            }

            for entity in entities.iter().rev() {
                self.remove_widget(*entity, &theme, ecm, &mut shell);
            }
        }
    }
}
