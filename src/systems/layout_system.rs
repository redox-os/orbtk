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
    layout::{get_constraint, Layout},
    properties::{Bounds, Constraint},
    structs::{Position, Size},
    theme::Theme,
};

pub enum LayoutResult {
    Size((f64, f64)),
    RequestChild(Entity, Constraint),
}

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

        let mut window_size = (0.0, 0.0);

        if let Ok(bounds) = ecm.borrow_component::<Bounds>(tree.root) {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let root = tree.children[&tree.root][0];

        self.layouts.borrow()[&root].measure(root, ecm, tree, &self.layouts);
        self.layouts.borrow()[&root].arrange(window_size, root, ecm, tree, &self.layouts);

        // fn layout_rec(
        //     ecm: &mut EntityComponentManager,
        //     tree: &Tree,
        //     constraint: &Constraint,
        //     entity: Entity,
        //     theme: &Theme,
        //     layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        // ) -> (f64, f64) {
        //     let mut size: Option<(f64, f64)> = None;

        //     loop {
        //         let layout_result = {
        //             let mut result = LayoutResult::Size((32.0, 32.0));
        //             if let Some(layout) = layouts.borrow().get(&entity) {
        //                 result =
        //                     layout.layout(entity, ecm, &constraint, &tree.children[&entity], size);
        //             }

        //             result
        //         };

        //         match layout_result {
        //             LayoutResult::Size(size) => {
        //                 println!("id: {}, width: {}, height: {}", entity, size.0, size.1);
        //                 if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
        //                     bounds.set_width(size.0);
        //                     bounds.set_height(size.1);
        //                 }

        //                 return size;
        //             }
        //             LayoutResult::RequestChild(child, child_bc) => {
        //                 if let Ok(visibility) = ecm.borrow_component::<Visibility>(entity) {
        //                     if *visibility == Visibility::Collapsed {
        //                         return (0.0, 0.0);
        //                     }
        //                 }
        //                 size = Some(layout_rec(ecm, tree, &child_bc, child, theme, layouts));
        //             }
        //         }
        //     }
        // }

        // if !self.update.get() {
        //     return;
        // }

        // println!("\n------ Start layout update ------\n");

        // let root = tree.root;

        // let mut backend = self.backend.borrow_mut();
        // let layout_context = backend.layout_context();

        // let constraint = {
        //     let bounds = if let Ok(bounds) = ecm.borrow_component::<Bounds>(root) {
        //         *bounds
        //     } else {
        //         Bounds::default()
        //     };

        //     // set windows constraint width and height to bounds width and height
        //     if let Ok(constraint) = ecm.borrow_mut_component::<Constraint>(root) {
        //         constraint.set_width(bounds.width());
        //         constraint.set_height(bounds.height());
        //         *constraint
        //     } else {
        //         Constraint::default()
        //     }
        // };

        // layout_rec(
        //     ecm,
        //     &tree,
        //     &constraint,
        //     tree.children[&root][0],
        //     &layout_context.theme,
        //     &self.layouts,
        // );

        println!("\n------ End layout update   ------\n");
    }
}
