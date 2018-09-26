use std::cell::RefCell;
use std::sync::Arc;
use std::collections::HashMap;

use dces::{Entity, EntityComponentManager, System};

use {Backend, RenderObject};

pub struct RenderSystem {
    pub backend: Arc<RefCell<Backend>>,
    pub render_objects: Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
}

impl System for RenderSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {   
        let mut backend = self.backend.borrow_mut();   
        let render_context = backend.render_context();
        render_context.renderer.render(&render_context.theme);

        for entity in entities {
            if let Some(render_object) = self.render_objects.borrow().get(entity) {
                render_object.render(*entity, ecm, render_context.renderer, &render_context.theme);
            }
        }
    }
}
