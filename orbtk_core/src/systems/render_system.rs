use std::collections::BTreeMap;

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, tree::Tree};

/// The `RenderSystem` iterates over all visual widgets.
///
/// For any widgets that have been marked dirty, new bounds have to be
/// recalculated. The resulting tree is rendered to the render buffer
/// which is then drawn to the screen.
#[derive(Constructor)]
pub struct RenderSystem {
    context_provider: ContextProvider,
}

impl System<Tree, RenderContext2D> for RenderSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree>,
        render_context: &mut RenderContext2D,
    ) {
        let root = ecm.entity_store().root();

        let dirty_widgets = ecm
            .component_store()
            .get::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .clone();

        // Only process, if
        // * there are `dirty` elements inside the entity vector
        // * context_provider it marked for first_run.
        if dirty_widgets.is_empty() && !self.context_provider.first_run.get() {
            return;
        }

        // reset the dirty flag of all dirty widgets to `false`
        for widget in dirty_widgets {
            if let Ok(dirty) = ecm.component_store_mut().get_mut::<bool>("dirty", widget) {
                *dirty = false;
            }
        }

        ecm.component_store_mut()
            .get_mut::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .clear();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root();
        let theme = ecm
            .component_store()
            .get::<Rc<Theme>>("theme", root)
            .unwrap()
            .clone();

        let mut offsets = BTreeMap::new();
        offsets.insert(root, (0.0, 0.0));

        // CONSOLE.time("render");

        render_context.start();
        render_context.begin_path();
        self.context_provider.render_objects.borrow()[&root].render(
            render_context,
            root,
            ecm,
            &self.context_provider,
            &theme,
            &mut offsets,
            debug,
        );
        render_context.finish();

        if self.context_provider.first_run.get() {
            self.context_provider.first_run.set(false);
        }
    }
}
