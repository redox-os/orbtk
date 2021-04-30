use std::collections::HashMap;

use legion::*;

#[derive(Clone, Debug)]
pub struct Tree {
    root: Entity,
    parent: HashMap<Entity, Entity>,
    children: HashMap<Entity, Vec<Entity>>,
}

impl Tree {
    /// Creates a new tree from entity.
    pub fn new(root: Entity) -> Self {
        let mut children = HashMap::new();
        children.insert(root, vec![]);

        Tree {
            root,
            parent: HashMap::new(),
            children,
        }
    }

    /// Returns `true` if the tree contains the given entity.
    pub fn contains(&self, entity: Entity) -> bool {
        self.children.contains_key(&entity)
    }

    /// Pushes a new entity to the tree.
    pub fn push(&mut self, entity: Entity, parent: Entity) {
        self.children.insert(entity, vec![]);
        self.parent.insert(entity, parent);
    }

    /// Returns the list of children for the given entity.
    pub fn children(&self, entity: Entity) -> Option<&Vec<Entity>> {
        self.children.get(&entity)
    }

    /// Returns the parent of the given entity.
    pub fn parent(&self, entity: Entity) -> Option<&Entity> {
        self.parent.get(&entity)
    }

    /// Returns the number of all entities in the tree.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns true if the tree has no entities.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}
