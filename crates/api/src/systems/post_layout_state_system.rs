use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, render::RenderContext2D, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
#[derive(Constructor)]
pub struct PostLayoutStateSystem {
    context_provider: ContextProvider,
    registry: Rc<RefCell<Registry>>,
}

impl PostLayoutStateSystem {
    fn remove_widget(
        &self,
        entity: Entity,
        theme: &Theme,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        {
            let render_objects = &self.context_provider.render_objects;
            let layouts = &self.context_provider.layouts;
            let handler_map = &self.context_provider.handler_map;
            let states = &self.context_provider.states;
            let event_queue = &self.context_provider.event_queue;
            let registry = &mut self.registry.borrow_mut();
            let new_states = &mut BTreeMap::new();

            let mut ctx = Context::new(
                (entity, ecm),
                &theme,
                render_objects,
                layouts,
                handler_map,
                states,
                new_states,
                event_queue,
                render_context,
            );

            if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&entity) {
                state.cleanup(registry, &mut ctx);
            }

            drop(ctx);
        }
        self.context_provider.states.borrow_mut().remove(&entity);

        ecm.remove_entity(entity);
        self.context_provider.layouts.borrow_mut().remove(&entity);
        self.context_provider
            .render_objects
            .borrow_mut()
            .remove(&entity);
        self.context_provider
            .handler_map
            .borrow_mut()
            .remove(&entity);
    }
}

impl System<Tree, StringComponentStore, RenderContext2D> for PostLayoutStateSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        // todo fix
        // if !self.shell.borrow().update() || !self.shell.borrow().running() {
        //     return;
        // }

        let root = ecm.entity_store().root();

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();
        let mut remove_widget_list: Vec<Entity> = vec![];

        {
            let mut keys = vec![];

            for key in self.context_provider.states.borrow().keys() {
                keys.push(*key);
            }

            for key in keys {
                {
                    let render_objects = &self.context_provider.render_objects;
                    let layouts = &self.context_provider.layouts;
                    let handler_map = &self.context_provider.handler_map;
                    let event_queue = &self.context_provider.event_queue;
                    let states = &self.context_provider.states;
                    let new_states = &mut BTreeMap::new();

                    let mut ctx = Context::new(
                        (key, ecm),
                        &theme,
                        render_objects,
                        layouts,
                        handler_map,
                        states,
                        new_states,
                        event_queue,
                        render_context,
                    );

                    self.context_provider.states.borrow_mut().get_mut(&key).unwrap().update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);   
                }

                for remove_widget in remove_widget_list.pop() {
                    let mut children = vec![];
                    get_all_children(&mut children, remove_widget, ecm.entity_store());

                    // remove children of target widget.
                    for entity in children.iter().rev() {
                        self.remove_widget(*entity, &theme, ecm, render_context);
                    }

                    // remove target widget
                    self.remove_widget(remove_widget, &theme, ecm, render_context);
                }
            }
        }
    }
}
