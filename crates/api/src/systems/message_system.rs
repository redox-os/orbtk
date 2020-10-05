use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, tree::Tree};

/// The `MessageSystem` is used to deliver messages to the `States` of a widget.
#[derive(Constructor)]
pub struct MessageSystem {
    context_provider: ContextProvider,
    registry: Rc<RefCell<Registry>>,
}

impl System<Tree, StringComponentStore, RenderContext2D> for MessageSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        let root = ecm.entity_store().root();

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let message_adapter = self.context_provider.message_adapter.clone();

        for entity in self.context_provider.message_adapter.entities() {
            let mut ctx = Context::new(
                (entity, ecm),
                &theme,
                &self.context_provider,
                render_context,
            );

            if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&entity) {
                state.messages(
                    message_adapter.message_reader(entity),
                    &mut self.registry.borrow_mut(),
                    &mut ctx,
                );
            } else {
                message_adapter.remove_message_for_entity(entity);
            }

            drop(ctx);
        }
    }
}
