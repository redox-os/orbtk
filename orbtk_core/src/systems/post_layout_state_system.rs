use std::{cell::RefCell, rc::Rc};

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, theming::Theme, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods
/// of widget states.
#[derive(Constructor)]
pub struct PostLayoutStateSystem {
    context_provider: ContextProvider,
    registry: Rc<RefCell<Registry>>,
}

impl PostLayoutStateSystem {
    fn remove_widget(
        &self,
        entity: Entity,
        theme: &Rc<Theme>,
        ecm: &mut EntityComponentManager<Tree>,
        render_context: &mut RenderContext2D,
    ) {
        {
            let mut ctx = Context::new(
                (entity, ecm),
                &theme,
                &self.context_provider,
                render_context,
            );

            if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&entity) {
                state.cleanup(&mut self.registry.borrow_mut(), &mut ctx);
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

impl System<Tree, RenderContext2D> for PostLayoutStateSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree>,
        render_context: &mut RenderContext2D,
    ) {
        // todo fix
        // if !self.shell.borrow().update() || !self.shell.borrow().running() {
        //     return;
        // }

        let root = ecm.entity_store().root();

        let theme = ecm
            .component_store()
            .get::<Rc<Theme>>("theme", root)
            .unwrap()
            .clone();

        let mut remove_widget_list: Vec<Entity> = vec![];

        {
            let mut keys = vec![];

            for key in self.context_provider.states.borrow().keys() {
                keys.push(*key);
            }

            for key in keys {
                if !*ecm.component_store().get::<bool>("dirty", key).unwrap() {
                    continue;
                }

                {
                    let mut ctx =
                        Context::new((key, ecm), &theme, &self.context_provider, render_context);

                    self.context_provider
                        .states
                        .borrow_mut()
                        .get_mut(&key)
                        .unwrap()
                        .update_post_layout(&mut *self.registry.borrow_mut(), &mut ctx);
                }

                while let Some(remove_widget) = remove_widget_list.pop() {
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
