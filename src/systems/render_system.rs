use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use {Backend, Rect, RenderObject, Tree};

pub struct RenderSystem {
    pub render_objects: Rc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
    pub backend: Rc<RefCell<Backend>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        let mut offsets = HashMap::new();
        offsets.insert(tree.root, (0, 0));

        // render window background
        render_context.renderer.render(&render_context.theme);

        for node in tree.into_iter() {
            let mut current_offset = (0, 0);

            if let Some(offset) = offsets.get(&tree.parent.get(&node).unwrap()) {
                current_offset = *offset;
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                render_object.render(
                    node,
                    ecm,
                    render_context.renderer,
                    &render_context.theme,
                    current_offset,
                );
            }

            if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                offsets.insert(
                    node,
                    (current_offset.0 + bounds.x, current_offset.1 + bounds.y),
                );
            }
        }
    }
}
