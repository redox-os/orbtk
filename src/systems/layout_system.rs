use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    application::Tree,
    backend::Backend,
    enums::Visibility,
    layout::Layout,
    properties::{Bounds, Constraint},
    theme::Theme,
};

pub enum LayoutResult {
    Size((u32, u32)),
    RequestChild(Entity, Constraint),
}

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calulated by the layout objects of layout widgets.
pub struct LayoutSystem {
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub backend: Rc<RefCell<dyn Backend>>,
    pub update: Rc<Cell<bool>>,
}

impl System<Tree> for LayoutSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        fn layout_rec(
            ecm: &mut EntityComponentManager,
            tree: &Tree,
            constraint: &Constraint,
            entity: Entity,
            theme: &Theme,
            layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        ) -> (u32, u32) {
            let mut size: Option<(u32, u32)> = None;

            loop {
                let layout_result = {
                    let mut result = LayoutResult::Size((32, 32));
                    if let Some(layout) = layouts.borrow().get(&entity) {
                        result = layout.layout(
                            entity,
                            ecm,
                            &constraint,
                            &tree.children[&entity],
                            size,
                            theme,
                        );
                    }

                    result
                };

                match layout_result {
                    LayoutResult::Size(size) => {
                        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
                            bounds.width = size.0;
                            bounds.height = size.1;
                        }

                        return size;
                    }
                    LayoutResult::RequestChild(child, child_bc) => {
                        if let Ok(visibility) = ecm.borrow_component::<Visibility>(entity) {
                            if *visibility == Visibility::Collapsed {
                                return (0, 0);
                            }
                        }
                        size = Some(layout_rec(ecm, tree, &child_bc, child, theme, layouts));
                    }
                }
            }
        }

        if !self.update.get() {
            return;
        }

        let root = tree.root;

        let mut backend = self.backend.borrow_mut();
        let layout_context = backend.layout_context();

        layout_rec(
            ecm,
            &tree,
            &Constraint {
                min_width: 0,
                min_height: 0,
                max_width: layout_context.window_size.0,
                max_height: layout_context.window_size.1,
                width: 0,
                height: 0,
            },
            root,
            &layout_context.theme,
            &self.layouts,
        );
    }
}
