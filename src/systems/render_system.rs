use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use application::Tree;
use backend::Backend;
use enums::Visibility;
use render_object::RenderObject;
use properties::{Offset, Point, Rect};
use theme::Selector;
use widget::WidgetContainer;

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    pub backend: Rc<RefCell<Backend>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() || tree.parent.is_empty() {
            return;
        }

        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        let parents = tree.parent.clone();
        let mut current_hidden_parent = None;

        let mut offsets = BTreeMap::new();
        offsets.insert(tree.root, (0, 0));

        // render window background
        render_context.renderer.render(render_context.theme.color("background", &"window".into()));

        for node in tree.into_iter() {
            let mut global_position = Point::default();

            if let Some(offset) = offsets.get(&tree.parent[&node]) {
                global_position = Point::new(offset.0, offset.1);
            }

            let offset = {
                // get offset from scrollable parent
                if let Ok(offset) = ecm.borrow_component::<Offset>(tree.parent[&node]) {
                    Point::new(offset.0, offset.1)
                } else {
                    Point::default()
                }
            };

            // Hide all children of a hidden parent
            if let Some(parent) = current_hidden_parent {
                if parent == parents[&node] {
                    current_hidden_parent = Some(node);
                    continue;
                } else {
                    current_hidden_parent = None;
                }
            } 

            // render debug border for each widget
            if self.debug_flag.get() {
                if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                    if let Ok(parent_bounds) = ecm.borrow_component::<Rect>(tree.parent[&node]) {
                        let selector = Selector::new().with("debugborder");

                        render_context.renderer.render_rectangle(
                            bounds,
                            parent_bounds,
                            &offset,
                            &global_position,
                            render_context.theme.uint("border-radius", &selector),
                            render_context.theme.color("background", &selector),
                            render_context.theme.uint("border-width", &selector),
                            render_context.theme.color("border-color", &selector),
                        );
                    }
                }
            }

            // hide hidden widget
            if let Ok(visibility) = ecm.borrow_component::<Visibility>(node) {
                if *visibility != Visibility::Visible {
                    current_hidden_parent = Some(node);
                     continue;
                }             
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                render_object.render(
                    render_context.renderer,
                    &WidgetContainer::new(node, ecm, tree),
                    &render_context.theme,
                    &offset,
                    &global_position,
                );
            }

            let mut global_pos = (0, 0);

            if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                global_pos = (global_position.x + bounds.x, global_position.y + bounds.y);
                offsets.insert(node, global_pos);
            }

            if let Ok(g_pos) = ecm.borrow_mut_component::<Point>(node) {
                g_pos.x = global_pos.0;
                g_pos.y = global_pos.1;
            }
        }
    }
}
