use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree, utils::*};

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calulated by the layout objects of layout widgets.
pub struct LayoutSystem {
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

impl System<Tree> for LayoutSystem {
    fn run(&self, tree: &mut Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() || !self.running.get() {
            return;
        }

        // if self.debug_flag.get() {
        //     shell::log("\n------ Start layout update  ------\n".to_string());
        // }

        let mut window_size = (0.0, 0.0);

        if let Ok(bounds) = ecm.borrow_component::<Bounds>(tree.root) {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let root = tree.children[&tree.root][0];

        let theme = ecm.borrow_component::<Theme>(tree.root).unwrap().0.clone();

        self.layouts.borrow()[&root].measure(
            self.shell.borrow_mut().render_context_2_d(),
            root,
            ecm,
            tree,
            &self.layouts,
            &theme,
        );
        self.layouts.borrow()[&root].arrange(
            self.shell.borrow_mut().render_context_2_d(),
            window_size,
            root,
            ecm,
            tree,
            &self.layouts,
            &theme,
        );

        // if self.debug_flag.get() {
        //     println!("\n------ End layout update   ------\n");
        // }
    }
}
