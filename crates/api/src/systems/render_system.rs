use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    css_engine::*,
    prelude::*,
    shell::{Shell, CONSOLE},
    tree::Tree,
    utils::Brush,
};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem;

impl System<Tree, StringComponentStore, ContextProvider<'_>> for RenderSystem {
    fn run_with_context(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>, ctx: &mut ContextProvider) {
        if !self.shell.borrow().update()
            || !self.shell.borrow().running()
            || ecm.entity_store().parent.is_empty()
        {
            return;
        }

        let mut shell = &mut self.shell.borrow_mut();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root();

        // sets the window background of the real window.
        if let Ok(background) = ecm.component_store().get::<Brush>("background", root) {
            if let Brush::SolidColor(color) = background {
                shell.set_background_color(color.r(), color.g(), color.b());
            }
        };

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut offsets = BTreeMap::new();
        offsets.insert(root, (0.0, 0.0));

        CONSOLE.time("render");

        shell.render_context_2_d().start();
        shell.render_context_2_d().begin_path();
        self.render_objects.borrow()[&root].render(
            &mut shell,
            root,
            ecm,
            &self.render_objects,
            &self.layouts,
            &self.handlers,
            &self.states,
            &theme,
            &mut offsets,
            debug,
        );
        shell.render_context_2_d().finish();

        //  print_tree(root, 0, ecm);
    }
}
