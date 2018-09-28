use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use {Constraint, Entity, EntityComponentManager, LayoutObject, Rect, System, Theme, Tree};

pub enum LayoutResult {
    Size((u32, u32)),
    RequestChild(Entity, Constraint),
}

pub struct LayoutSystem {
    pub tree: Arc<RefCell<Tree>>,
    pub theme: Arc<Theme>,
    pub layout_objects: Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
}

impl System for LayoutSystem {
    fn run(&self, _entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        fn layout_rec(
            ecm: &mut EntityComponentManager,
            tree: &Arc<RefCell<Tree>>,
            constraint: &Constraint,
            entity: Entity,
            theme: &Arc<Theme>,
            layout_objects: &Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
        ) -> (u32, u32) {
            let mut size: Option<(u32, u32)> = None;

            loop {
                let mut children_pos = None;
                let layout_result = {
                    let mut result = LayoutResult::Size((32, 32));
                    if let Some(layout) = layout_objects.borrow().get(&entity) {
                        result = layout.layout(
                            entity,
                            ecm,
                            constraint,
                            &tree.borrow().children.get(&entity).unwrap(),
                            &mut children_pos,
                            size,
                            theme,
                        );
                    }

                    result
                };

                if let Some(children_pos) = children_pos {
                    for (entity, pos) in children_pos {
                        if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(entity) {
                            bounds.x = pos.0;
                            bounds.y = pos.1;
                        }
                    }
                }

                match layout_result {
                    LayoutResult::Size(size) => {
                        if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(entity) {
                            bounds.width = size.0;
                            bounds.height = size.1;
                        }

                        return size;
                    }
                    LayoutResult::RequestChild(child, child_bc) => {
                        size = Some(layout_rec(
                            ecm,
                            tree,
                            &child_bc,
                            child,
                            theme,
                            layout_objects,
                        ));
                    }
                }
            }
        }

        let root = self.tree.borrow().root;

        // todo: use widnow size!!!
        layout_rec(
            ecm,
            &self.tree,
            &Constraint {
                min_width: 0,
                min_height: 0,
                max_width: 400,
                max_height: 300,
            },
            root,
            &self.theme,
            &self.layout_objects,
        );
    }
}
