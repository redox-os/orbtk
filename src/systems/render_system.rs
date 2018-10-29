use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use render_object::RenderObject;
use structs::{Point, Rect};
use tree::Tree;
use widget::WidgetContainer;

pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    pub backend: Rc<RefCell<Backend>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        let mut offsets = BTreeMap::new();
        offsets.insert(tree.root, (0, 0));

        // render window background
        render_context.renderer.render(&render_context.theme);

        for node in tree.into_iter() {
            let mut current_offset = (0, 0);
            let mut boundery = (0, 0);

            if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(tree.parent[&node]) {
                boundery = (bounds.width, bounds.height);
            }

            if let Some(offset) = offsets.get(&tree.parent.get(&node).unwrap()) {
                current_offset = *offset;
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                render_object.render(
                    render_context.renderer,
                    &WidgetContainer::new(node, ecm, tree),
                    &render_context.theme,
                    boundery,
                    current_offset,
                );
            }

            let mut global_pos = (0, 0);

            if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                global_pos = (current_offset.0 + bounds.x, current_offset.1 + bounds.y);
                offsets.insert(node, global_pos);
            }

            if let Ok(g_pos) = ecm.borrow_mut_component::<Point>(node) {
                g_pos.x = global_pos.0;
                g_pos.y = global_pos.1;
            }
        }
    }
}
