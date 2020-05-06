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
            let render_objects = &self.context_provider.render_objects;
            let layouts = &self.context_provider.layouts;
            let handler_map = &self.context_provider.handler_map;
            let states = &self.context_provider.states;
            let event_queue = &self.context_provider.event_queue;
            let new_states = &mut BTreeMap::new();

            let mut ctx = Context::new(
                (root, ecm),
                &theme,
                render_objects,
                layouts,
                handler_map,
                states,
                new_states,
                event_queue,
                render_context,
            );

            for (node, state) in &mut *self.context_provider.states.borrow_mut() {
                ctx.entity = *node;

                state.update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);
                remove_widget_list.append(ctx.remove_widget_list());
            }
        }

        for child in remove_widget_list {
            let mut entities = vec![child];

            for child in &ecm.entity_store().children[&child] {
                entities.push(*child);
            }

            for entity in entities.iter().rev() {
                self.remove_widget(*entity, &theme, ecm, render_context);
            }
        }
    }
}
