use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    application::Tree, backend::Backend, layout::Layout, properties::Bounds, structs::Size,
};

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calulated by the layout objects of layout widgets.
pub struct LayoutSystem {
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub backend: Rc<RefCell<dyn Backend>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

impl System<Tree> for LayoutSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.running.get() {
            return;
        }

        if self.debug_flag.get() {
            println!("\n------ Start layout update  ------\n");
        }

        let mut window_size = (0.0, 0.0);

        if let Ok(bounds) = ecm.borrow_component::<Bounds>(tree.root) {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let root = tree.children[&tree.root][0];

        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        self.layouts.borrow()[&root].measure(root, ecm, tree, &self.layouts, &render_context.theme);
        self.layouts.borrow()[&root].arrange(
            window_size,
            root,
            ecm,
            tree,
            &self.layouts,
            &render_context.theme,
        );

        if self.debug_flag.get() {
            println!("\n------ End layout update   ------\n");
        }
    }
}
