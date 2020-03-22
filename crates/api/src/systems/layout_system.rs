use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::WindowShell, tree::Tree, utils::*};

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calculated by the layout objects of layout widgets.
pub struct LayoutSystem {
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
}

impl System<Tree, StringComponentStore> for LayoutSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        if !self.shell.borrow().update() || !self.shell.borrow().running() {
            return;
        }

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

        self.layouts.borrow()[&root].measure(
            self.shell.borrow_mut().render_context_2_d(),
            root,
            ecm,
            &self.layouts.borrow(),
            &theme,
        );

        self.layouts.borrow()[&root].arrange(
            self.shell.borrow_mut().render_context_2_d(),
            window_size,
            root,
            ecm,
            &self.layouts.borrow(),
            &theme,
        );

        // if self.debug_flag.get() {
        //     println!("\n------ End layout update   ------\n");
        // }
    }
}
