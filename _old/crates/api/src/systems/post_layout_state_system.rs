use dces::prelude::*;

use crate::{prelude::*, theming::Theme, tree::Tree};

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
#[derive(Constructor)]
pub struct PostLayoutStateSystem {
    context_provider: ContextProvider,
}

impl PostLayoutStateSystem {
    fn remove_widget(
        &self,
        entity: Entity,
        theme: &Theme,
        ecm: &mut EntityComponentManager<Tree>,
        res: &mut Resources,
    ) {
        {
            let mut ctx = Context::new((entity, ecm), &theme, &self.context_provider);

            if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&entity) {
                state.cleanup(&mut ctx, res);
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

impl System<Tree> for PostLayoutStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>, res: &mut Resources) {
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
                if !*ecm.component_store().get::<bool>("dirty", key).unwrap() {
                    continue;
                }

                {
                    let mut ctx = Context::new((key, ecm), &theme, &self.context_provider);

                    self.context_provider
                        .states
                        .borrow_mut()
                        .get_mut(&key)
                        .unwrap()
                        .update_post_layout(&mut ctx, res);
                }

                while let Some(remove_widget) = remove_widget_list.pop() {
                    let mut children = vec![];
                    get_all_children(&mut children, remove_widget, ecm.entity_store());

                    // remove children of target widget.
                    for entity in children.iter().rev() {
                        self.remove_widget(*entity, &theme, ecm, res);
                    }

                    // remove target widget
                    self.remove_widget(remove_widget, &theme, ecm, res);
                }
            }
        }
    }
}
