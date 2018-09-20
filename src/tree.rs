use std::collections::BTreeMap;

use {Entity, NotFound};

#[derive(Default)]
pub struct Tree {
    pub root: Entity,
    pub children: BTreeMap<Entity, Vec<Entity>>,
    parent: BTreeMap<Entity, Entity>,
}

impl Tree {
    pub fn register_node(&mut self, entity: Entity) {
        self.children.insert(entity, vec![]);
        self.parent.insert(entity, entity);
    }

    pub fn append_child(&mut self, parent: Entity, child: Entity) -> Result<Entity, NotFound> {
        if let Some(p) = self.children.get_mut(&parent) {
            p.push(child);
        } else {
            return Err(NotFound::Parent(parent));
        }

        self.parent.insert(child, parent);

        Ok(child)
    }
}
