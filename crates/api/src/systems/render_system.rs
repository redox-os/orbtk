use std::collections::BTreeMap;

use dces::prelude::{EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, render::RenderContext2D, tree::Tree};

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
        // if !self.shell.borrow().update()
        //     || !self.shell.borrow().running()
        //     || ecm.entity_store().parent.is_empty()
        // {
        //     return;
        // }

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root();

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
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

        //  print_tree(root, 0, ecm);
    }
}
