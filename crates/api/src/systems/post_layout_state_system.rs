use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::Shell, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem;

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
    fn run_with_context(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>, ctx: &mut ContextProvider) {
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
            let mut keys = vec![];

            for key in self.states.borrow().keys() {
                keys.push(*key);
            }

            for key in keys {
                {
                    let render_objects = &self.render_objects;
                    let layouts = &mut self.layouts.borrow_mut();
                    let handlers = &mut self.handlers.borrow_mut();
                    let new_states = &mut BTreeMap::new();

                    let mut ctx = Context::new(
                        (key, ecm),
                        shell,
                        &theme,
                        render_objects,
                        layouts,
                        handlers,
                        &self.states,
                        new_states,
                    );

                    self.states.borrow_mut().get_mut(&key).unwrap().update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);   
                }

                for remove_widget in remove_widget_list.pop() {
                    let mut children = vec![];
                    get_all_children(&mut children, remove_widget, ecm.entity_store());

                    // remove children of target widget.
                    for entity in children.iter().rev() {
                        self.remove_widget(*entity, &theme, ecm, &mut shell);
                    }

                    // remove target widget
                    self.remove_widget(remove_widget, &theme, ecm, &mut shell);
                }
            }
        }
    }
}
