use std::collections::BTreeMap;

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, tree::Tree};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
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

        if dirty_widgets.is_empty() && !self.context_provider.first_run.get() {
            return;
        }

        let dirty_widgets_clone = dirty_widgets.clone();

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

        let mut dirty_region: Option<Rectangle> = None;
        // Get the dirty region of every dirty widget
        for widget in dirty_widgets_clone {
            if let Ok(Some(widget_dirty_region)) = ecm
                .component_store()
                .get::<Option<Rectangle>>("dirty_region", widget)
            {
                println!(
                    "vmx: core: sytems: render systems: entity, dirty region2: {:?} {:?}",
                    widget, widget_dirty_region
                );
                // Expand the dirty region if there was already one set.
                match dirty_region {
                    Some(mut dr) => dr.join_with_rectangle(widget_dirty_region),
                    None => dirty_region = Some(widget_dirty_region.clone()),
                }
            }
        }

        // Store the dirty region in the root
        if let Ok(store_dirty_region) = ecm
            .component_store_mut()
            .get_mut::<Option<Rectangle>>("dirty_region", root)
        {
            *store_dirty_region = dirty_region;
        }
    }
}
