use std::collections::BTreeMap;

use dces::prelude::{EntityComponentManager, System};

use crate::{prelude::*, render::RenderContext2D, tree::Tree};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
#[derive(Constructor)]
pub struct RenderSystem {
    context_provider: ContextProvider,
}

impl System<Tree, StringComponentStore, RenderContext2D> for RenderSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        let root = ecm.entity_store().root();

        let mut dirty_widgets = ecm
            .component_store()
            .get::<HashSet<Entity>>("dirty_widgets", root)
            .unwrap()
            .clone();

        for widget in dirty_widgets.clone() {
            if let Ok(dirty) = ecm.component_store_mut().get_mut::<bool>("dirty", widget) {
                *dirty = true;
            } else {
                dirty_widgets.remove(&widget);
            }
        }

        if dirty_widgets.is_empty() {
            return;
        }

        ecm.component_store_mut()
            .get_mut::<HashSet<Entity>>("dirty_widgets", root)
            .unwrap()
            .clear();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root();

        let theme = ecm
            .component_store()
            .get::<Global>("global", root)
            .unwrap()
            .theme
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
    }
}
