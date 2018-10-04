use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{mpsc::Sender, Arc};

use dces::{Entity, EntityComponentManager, System};

use {Label, Rect, RenderContainer, RenderObject, Selector, Tree};

pub struct RenderSystem {
    pub render_objects: Arc<RefCell<HashMap<Entity, Arc<RenderObject>>>>,
    pub render_sender: Sender<Vec<RenderContainer>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut render_vec = vec![];

        let mut offsets = HashMap::new();
        offsets.insert(tree.root, (0, 0));

        for node in tree.into_iter() {
            let mut current_offset = (0, 0);

            if let Some(offset) = offsets.get(&tree.parent.get(&node).unwrap()) {
                current_offset = *offset;
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
                    if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                        // todo: find better solution for content handling
                        let content: Option<Arc<Any + Send + Sync>> = {
                            if let Ok(label) = ecm.borrow_component::<Label>(node) {
                                Some(Arc::new(label.0.clone()))
                            } else {
                                None
                            }
                        };

                        render_vec.push(RenderContainer {
                            bounds: bounds.clone(),
                            selector: selector.clone(),
                            render_object: render_object.clone(),
                            offset: current_offset,
                            content,
                        })
                    }
                }
            }

            if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                offsets.insert(
                    node,
                    (current_offset.0 + bounds.x, current_offset.1 + bounds.y),
                );
            }
        }

        if let Err(err) = self.render_sender.send(render_vec) {
            println!("Render System: {}", err);
        }
    }
}
