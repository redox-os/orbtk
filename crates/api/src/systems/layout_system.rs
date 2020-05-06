use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, render::RenderContext2D, tree::Tree, utils::*};

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calculated by the layout objects of layout widgets.
#[derive(Constructor)]
pub struct LayoutSystem {
    context_provider: ContextProvider,
}

impl System<Tree, StringComponentStore, RenderContext2D> for LayoutSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_context: &mut RenderContext2D,
    ) {
        // todo fix.
        // if !self.shell.borrow().update() || !self.shell.borrow().running() {
        //     return;
        // }

        // if self.debug_flag.get() {
        //     shell::log("\n------ Start layout update  ------\n".to_string());
        // }

        let mut window_size = (0.0, 0.0);
        let root = ecm.entity_store().root();

        if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", root) {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        self.context_provider.layouts.borrow()[&root].measure(
            render_context,
            root,
            ecm,
            &self.context_provider.layouts.borrow(),
            &theme,
        );

        self.context_provider.layouts.borrow()[&root].arrange(
            render_context,
            window_size,
            root,
            ecm,
            &self.context_provider.layouts.borrow(),
            &theme,
        );

        // if self.debug_flag.get() {
        //     println!("\n------ End layout update   ------\n");
        // }
    }
}
