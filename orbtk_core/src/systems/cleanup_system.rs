use std::{cell::RefCell, rc::Rc};

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, theming::Theme, tree::Tree};

/// Handles the inner cleanup while window is closing.
#[derive(Constructor)]
pub struct CleanupSystem {
    context_provider: ContextProvider,
    registry: Rc<RefCell<Registry>>,
}

impl System<Tree, RenderContext2D> for CleanupSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree>,
        render_context: &mut RenderContext2D,
    ) {
        // let mut shell = self.shell.borrow_mut();
        let root = ecm.entity_store().root();
        let theme = ecm
            .component_store()
            .get::<Rc<Theme>>("theme", root)
            .unwrap()
            .clone();

        let mut dirty_index = 0;

        loop {
            if dirty_index
                >= ecm
                    .component_store()
                    .get::<Vec<Entity>>("dirty_widgets", root)
                    .unwrap()
                    .len()
            {
                break;
            }

            let skip = false;

            let widget = *ecm
                .component_store()
                .get::<Vec<Entity>>("dirty_widgets", root)
                .unwrap()
                .get(dirty_index)
                .unwrap();

            let mut keys = vec![];

            if !skip {
                let registry = &mut self.registry.borrow_mut();

                let mut ctx = Context::new(
                    (widget, ecm),
                    &theme,
                    &self.context_provider,
                    render_context,
                );

                if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&widget) {
                    state.cleanup(registry, &mut ctx);
                }

                keys.append(&mut ctx.new_states_keys());
            }

            dirty_index += 1;
        }
    }
}
