use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use dces::{Entity, EntityComponentManager, System};

use {Backend, Rect, RenderObject, Tree, RenderContext};

pub struct RenderSystem {
    pub tree: Arc<RefCell<Tree>>,
    pub backend: Arc<RefCell<Backend>>,
    pub render_objects: Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
}

impl System for RenderSystem {
    fn run(&self, _entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {

        fn render(tree: &Arc<RefCell<Tree>>, root: Entity, pos: (i32, i32),ecm: &mut EntityComponentManager,render_objects: &Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>, render_context: &mut RenderContext) {
            if let Some(render_object) = render_objects.borrow().get(&root) {
                render_object.render(
                    root,
                    ecm,
                    render_context.renderer,
                    &render_context.theme,
                    pos,
                );
            }

            let mut current_pos = pos;

            if let Ok(bounds) = ecm.borrow_component::<Rect>(root) {
                current_pos.0 += bounds.x;
                current_pos.1 += bounds.y;
            }

            for entity in tree.borrow().children.get(&root).unwrap(){
                render(tree, *entity, current_pos, ecm, render_objects, render_context);
            }
        }


        let mut backend = self.backend.borrow_mut();
        let mut render_context = backend.render_context();
        render_context.renderer.render(&render_context.theme);

        render(&self.tree, self.tree.borrow().root, (0, 0), ecm, &self.render_objects, &mut render_context);
    }
}
