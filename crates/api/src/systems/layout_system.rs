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

impl System<Tree, StringComponentStore> for LayoutSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        if !self.update.get() || !self.running.get() {
            return;
        }

        // if self.debug_flag.get() {
        //     shell::log("\n------ Start layout update  ------\n".to_string());
        // }

        let mut window_size = (0.0, 0.0);
        let root = ecm.entity_store().root;

        if let Ok(bounds) = ecm
            .component_store()
            .borrow_component::<Rectangle>("bounds", root)
        {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let theme = ecm
            .component_store()
            .borrow_component::<Theme>("theme", root)
            .unwrap()
            .0
            .clone();

        self.layouts.borrow()[&root].measure(
            self.shell.borrow_mut().render_context_2_d(),
            root,
            ecm,
            &self.layouts,
            &theme,
        );

        self.layouts.borrow()[&root].arrange(
            self.shell.borrow_mut().render_context_2_d(),
            window_size,
            root,
            ecm,
            &self.layouts,
            &theme,
        );

        // if self.debug_flag.get() {
        //     println!("\n------ End layout update   ------\n");
        // }
    }
}
