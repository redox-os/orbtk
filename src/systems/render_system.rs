use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::{Entity, EntityComponentManager, System};

use crate::{
    application::Tree,
    core::Backend,
    shape_renderer::ShapeRenderer,
    enums::Visibility,
    properties::{Bounds, Point},
    theme::{Selector, UpdateableShape},
};

use orbclient::Color;

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub shapes: Rc<RefCell<BTreeMap<Entity, Box<dyn UpdateableShape>>>>,
    pub backend: Rc<RefCell<dyn Backend>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() || tree.parent.is_empty() {
            return;
        }

        let mut backend = self.backend.borrow_mut();

        let parents = tree.parent.clone();
        let mut current_hidden_parent = None;

        let mut offsets = BTreeMap::new();

        offsets.insert(tree.root, (0, 0));

        // render window background
        {
            let render_context = backend.render_context();
            // render_context.fill_background();

            for node in tree.into_iter() {
                let mut global_position = Point::default();

                // remove dirty flags from selectors.
                if let Ok(selector) = ecm.borrow_mut_component::<Selector>(node) {
                    selector.set_dirty(false);
                }

                if let Some(offset) = offsets.get(&tree.parent[&node]) {
                    global_position = Point::new(offset.0, offset.1);
                }

                // Hide all children of a hidden parent
                if let Some(parent) = current_hidden_parent {
                    if parent == parents[&node] {
                        current_hidden_parent = Some(node);
                        continue;
                    } else {
                        current_hidden_parent = None;
                    }
                }

                // hide hidden widget
                if let Ok(visibility) = ecm.borrow_component::<Visibility>(node) {
                    if *visibility != Visibility::Visible {
                        current_hidden_parent = Some(node);
                        continue;
                    }
                }

                if let Some(shape) = self.shapes.borrow_mut().get_mut(&node) {
                    if let Ok(bounds) = ecm.borrow_component::<Bounds>(node) {

                        shape.update_by_bounds(
                            (global_position.x + bounds.x) as f64,
                            (global_position.y + bounds.y) as f64,
                            bounds.width as f64,
                            bounds.height as f64,
                        );

                        render_context.renderer.render_path(&mut shape.path());

                        // draw debug border
                        if self.debug_flag.get() {
                            render_context
                                .renderer
                                .set_stroke_style(Color::rgb(0, 0, 255));
                            render_context.renderer.rect(
                                global_position.x as f64,
                                global_position.y as f64,
                                bounds.width as f64,
                                bounds.height as f64,
                            );
                            render_context.renderer.stroke();
                        }
                    }
                }

                let mut global_pos = (0, 0);

                if let Ok(bounds) = ecm.borrow_component::<Bounds>(node) {
                    global_pos = (global_position.x + bounds.x, global_position.y + bounds.y);
                    offsets.insert(node, global_pos);
                }

                if let Ok(g_pos) = ecm.borrow_mut_component::<Point>(node) {
                    g_pos.x = global_pos.0;
                    g_pos.y = global_pos.1;
                }
            }
        }

        backend.flip();
    }
}
