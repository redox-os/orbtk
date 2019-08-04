/*!

This crate provides a index (entity) based tree structure compatible to the [DCES](https://gitlab.redox-os.org/redox-os/dces-rust)
Entity Component System. The tree could be used as entity storage.

# Example

Basic usage of the tree:

```rust,no_run

use orbtk_tree::prelude::*;

let mut tree = Tree::new();
tree.register_node(0);
tree.register_node(1);
tree.append_child(0, 1);
```

 */

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::{Entity, EntityStore};

pub mod prelude;

/// Used as return type if a requested entity is not found on the tree.
#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    /// Parent could not be found.
    Parent(Entity),

    /// Child could not be found
    Child(Entity),
}

/// Base data structure to manage the entity entities of a window in a tree based structure.
#[derive(Clone, Default, Debug)]
pub struct Tree {
    pub root: Entity,
    pub children: BTreeMap<Entity, Vec<Entity>>,
    pub parent: BTreeMap<Entity, Option<Entity>>,
    iterator_start_node: Cell<Entity>,
}

impl Tree {
    /// Creates a new tree with default values.
    pub fn new() -> Self {
        Tree::default()
    }

    /// Configure the tree iterator with a start node.
    pub fn start_node(&self, start_node: impl Into<Entity>) -> &Self {
        self.iterator_start_node.set(start_node.into());
        self
    }

    /// Registers a new entity `entity` as node.
    pub fn register_node(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.children.insert(entity, vec![]);
        self.parent.insert(entity, None);
    }

    /// Sets the root.
    pub fn set_root(&mut self, root: impl Into<Entity>) {
        self.root = root.into();
        self.iterator_start_node.set(self.root);
    }

    /// Appends a `child` entity to the given `parent` entity.
    /// Raised `NotFound` error if the parent is not part of the tree.
    pub fn append_child(
        &mut self,
        parent: impl Into<Entity>,
        child: impl Into<Entity>,
    ) -> Result<Entity, NotFound> {
        let parent = parent.into();
        let child = child.into();
        if let Some(p) = self.children.get_mut(&parent) {
            p.push(child);
        } else {
            return Err(NotFound::Parent(parent));
        }

        self.parent.insert(child, Some(parent));

        Ok(child)
    }

    /// Returns the number of all entities in the tree.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns true if the tree has no entities.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn request_start_node(&self) -> Entity {
        let start_node = self.iterator_start_node.get();
        self.iterator_start_node.set(self.root);
        start_node
    }
}

impl EntityStore for Tree {
    fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.register_node(entity.into());
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.children.remove(&entity);
        self.parent.remove(&entity);
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = Entity;
    type IntoIter = TreeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let start_node = self.request_start_node();

        TreeIterator {
            tree: self,
            path: RefCell::new(vec![]),
            start_node,
        }
    }
}

/// Used to create an iterator for the tree.
pub struct TreeIterator<'a> {
    tree: &'a Tree,
    path: RefCell<Vec<Entity>>,
    start_node: Entity,
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let mut path = self.path.borrow_mut();
        let mut result = None;

        if path.is_empty() {
            result = Some(self.start_node);
        } else {
            let mut current_node = path[path.len() - 1];

            // if current node has children return the first child
            if !self.tree.children[&current_node].is_empty() {
                result = Some(self.tree.children[&current_node][0]);
            } else {
                // if the node doesn't have kids check its siblings
                loop {
                    path.pop();

                    if path.is_empty() {
                        break;
                    }

                    let parent = self.tree.parent[&current_node];
                    let siblings = &self.tree.children[&parent.unwrap()];
                    let sibling_index =
                        siblings.iter().position(|&r| r == current_node).unwrap() + 1;

                    if sibling_index < siblings.len() {
                        result = Some(siblings[sibling_index]);
                        break;
                    } else {
                        current_node = parent.unwrap();
                    }
                }
            }
        }

        if let Some(result) = result {
            path.push(result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use dces::prelude::*;

    use super::*;

    #[test]
    fn test_register_node() {
        let mut tree = Tree::new();
        tree.register_node(0);

        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.parent.len(), 1);
        assert!(tree.children.get(&Entity(0)).is_some());
    }

    #[test]
    fn test_append_child() {
        let parent = Entity(0);
        let child = Entity(1);

        let mut tree = Tree::new();
        tree.register_node(parent);
        tree.register_node(child);
        tree.append_child(parent, child).unwrap();

        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.parent.len(), 2);
        assert_eq!(tree.children.get(&parent).unwrap()[0], child);
        assert_eq!(tree.parent.get(&child).unwrap().unwrap(), parent);
    }

    #[test]
    fn test_len() {
        let mut tree = Tree::new();
        assert_eq!(tree.children.len(), 0);

        tree.register_node(0);
        assert_eq!(tree.children.len(), 1);

        tree.register_node(1);
        assert_eq!(tree.children.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut tree = Tree::new();
        assert!(tree.is_empty());

        tree.register_node(0);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_register_entity() {
        let mut tree = Tree::new();
        tree.register_entity(0);

        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.parent.len(), 1);
        assert!(tree.children.get(&Entity(0)).is_some());
    }

    #[test]
    fn test_remove_entity() {
        let mut tree = Tree::new();

        tree.register_entity(0);
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.parent.len(), 1);
        assert!(tree.children.get(&Entity(0)).is_some());

        tree.remove_entity(0);
        assert_eq!(tree.children.len(), 0);
        assert_eq!(tree.parent.len(), 0);
        assert!(tree.children.get(&Entity(0)).is_none());
    }

    #[test]
    fn test_iterator_next() {
        let mut tree = Tree::new();
        tree.register_entity(0);
        tree.register_entity(1);
        tree.register_entity(2);
        tree.register_entity(3);
        tree.register_entity(4);
        tree.register_entity(5);
        tree.register_entity(6);

        tree.append_child(0, 1).unwrap();
        tree.append_child(0, 2).unwrap();

        tree.append_child(1, 3).unwrap();
        tree.append_child(1, 4).unwrap();

        tree.append_child(2, 5).unwrap();
        tree.append_child(2, 6).unwrap();

        let mut iterator = tree.into_iter();

        assert_eq!(Entity(0), iterator.next().unwrap());
        assert_eq!(Entity(1), iterator.next().unwrap());
        assert_eq!(Entity(3), iterator.next().unwrap());
        assert_eq!(Entity(4), iterator.next().unwrap());
        assert_eq!(Entity(2), iterator.next().unwrap());
        assert_eq!(Entity(5), iterator.next().unwrap());
        assert_eq!(Entity(6), iterator.next().unwrap());
    }
}
