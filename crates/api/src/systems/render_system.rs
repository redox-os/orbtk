use std::{cell::RefCell, collections::BTreeMap, rc::Rc, any::Any};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    css_engine::*,
    prelude::*,
    shell::{WindowShell, CONSOLE},
    tree::Tree,
};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
}

impl System<Tree, StringComponentStore> for RenderSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
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

        let root = ecm.entity_store().root;

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
