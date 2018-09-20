use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use dces::{Entity, EntityComponentManager, System};

use {Rect, Tree};

pub struct Layout {
    pub layout_fn: Box<
        Fn(Entity, &EntityComponentManager, &BoxConstraints, &[Entity], &mut HashMap<Entity, (i32, i32)>, Option<(u32, u32)>)
            -> LayoutResult,
    >,
}

impl Layout {
    pub fn new(
        layout_fn: Box<
            Fn(
                Entity,
                &EntityComponentManager,
                &BoxConstraints,
                &[Entity],
                &mut HashMap<Entity, (i32, i32)>,
                Option<(u32, u32)>,
            ) -> LayoutResult,
        >,
    ) -> Self {
        Layout { layout_fn }
    }

    pub fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        bc: &BoxConstraints,
        children: &[Entity],
        children_pos: &mut HashMap<Entity, (i32, i32)>,
        size: Option<(u32, u32)>,
    ) -> LayoutResult {
        (self.layout_fn)(entity, ecm, bc, children, children_pos, size)
    }
}

#[derive(Clone, Copy)]
pub struct BoxConstraints {
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

impl BoxConstraints {
    pub fn tight(size: (u32, u32)) -> BoxConstraints {
        BoxConstraints {
            min_width: size.0,
            max_width: size.0,
            min_height: size.1,
            max_height: size.1,
        }
    }

    pub fn constrain(&self, size: (u32, u32)) -> (u32, u32) {
        (
            clamp(size.0, self.min_width, self.max_width),
            clamp(size.1, self.min_height, self.max_height),
        )
    }
}

fn clamp(val: u32, min: u32, max: u32) -> u32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub enum LayoutResult {
    Size((u32, u32)),
    RequestChild(Entity, BoxConstraints),
}

pub struct LayoutSystem {
    pub tree: Arc<RefCell<Tree>>,
}

impl System for LayoutSystem {
    fn run(&self, _entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        fn layout_rec(
            ecm: &mut EntityComponentManager,
            tree: &Arc<RefCell<Tree>>,
            bc: &BoxConstraints,
            entity: Entity,
        ) -> (u32, u32) {
            let mut size = None;
            loop {
                let mut children_pos = HashMap::new();
                let layout_result = {
                    let mut result = LayoutResult::Size((32, 32));
                    if let Ok(layout) = ecm.borrow_component::<Layout>(entity) {
                        result = layout.layout(
                            entity,
                            ecm,
                            bc,
                            &tree.borrow().children.get(&entity).unwrap(),
                            &mut children_pos,
                            size,
                        );
                    }

                    result
                };

                for (entity, pos) in children_pos {
                    if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(entity) {
                        bounds.x = pos.0;
                        bounds.y = pos.1;
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
                        size = Some(layout_rec(ecm, tree, &child_bc, child));
                    }
                }
            }
        }

        let root = self.tree.borrow().root;

        // todo: use widnow size!!!
        layout_rec(ecm, &self.tree, &BoxConstraints::tight((200, 200)), root);
    }
}
