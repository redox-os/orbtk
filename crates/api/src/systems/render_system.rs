use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    prelude::*,
    shell::{log, WindowShell},
    tree::Tree,
    utils::*,
};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        if !self.update.get() || ecm.entity_store().parent.is_empty() || !self.running.get() {
            return;
        }

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root;

        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();

        let mut hidden_parents: HashSet<Entity> = HashSet::new();

        let mut offsets = BTreeMap::new();
        offsets.insert(root, (0.0, 0.0));

        let mut current_node = root;

        loop {
            let mut global_position = Point::default();

            if let Some(parent) = ecm.entity_store().parent[&current_node] {
                if let Some(offset) = offsets.get(&parent) {
                    global_position = Point::new(offset.0, offset.1);
                }
            }

            // Hide all children of a hidden parent
            if let Some(parent) = ecm.entity_store().parent[&current_node] {
                if hidden_parents.contains(&parent) {
                    hidden_parents.insert(current_node);
                    continue;
                }
            }

            // hide hidden widget
            if let Ok(visibility) = ecm
                .component_store()
                .borrow_component::<Visibility>(current_node)
            {
                if visibility.0 != VisibilityValue::Visible {
                    hidden_parents.insert(current_node);
                    continue;
                }
            }

            if let Some(render_object) = self.render_objects.borrow().get(&current_node) {
                render_object.render(
                    &mut Context::new(
                        current_node,
                        ecm,
                        &mut self.shell.borrow_mut(),
                        &theme,
                        self.render_objects.clone(),
                        self.layouts.clone(),
                        self.handlers.clone(),
                        self.states.clone(),
                    ),
                    &global_position,
                );
            }

            // render debug border for each widget
            if debug {
                if let Ok(bounds) = ecm
                    .component_store()
                    .borrow_component::<Bounds>(current_node)
                {
                    let selector = Selector::from("debug-border");
                    let brush = theme.brush("border-color", &selector.0).unwrap();
                    self.shell
                        .borrow_mut()
                        .render_context_2_d()
                        .set_stroke_style(brush);
                    self.shell.borrow_mut().render_context_2_d().stroke_rect(
                        global_position.x + bounds.x(),
                        global_position.y + bounds.y(),
                        bounds.width(),
                        bounds.height(),
                    );
                }
            }

            let mut global_pos = (0.0, 0.0);

            if let Ok(bounds) = ecm
                .component_store()
                .borrow_component::<Bounds>(current_node)
            {
                global_pos = (
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                );
                offsets.insert(current_node, global_pos);
            }

            if let Ok(g_pos) = ecm
                .component_store_mut()
                .borrow_mut_component::<Point>(current_node)
            {
                g_pos.x = global_pos.0;
                g_pos.y = global_pos.1;
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }

        self.shell.borrow_mut().flip();
    }
}
