/*!

This module provides a index (entity) based tree structure compatible to the [DCES](https://gitlab.redox-os.org/redox-os/dces-rust)
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

use std::{cell::Cell, collections::BTreeMap};

use dces::{entity::EntityStore, prelude::Entity};

/// Used as return type if a requested entity is not found on the tree.
#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    /// Parent could not be found.
    Parent(Entity),

    /// Child could not be found
    Child(Entity),
}

/// Base data structure to manage the entities of a window in a tree based structure.
#[derive(Clone, Default, Debug)]
pub struct Tree {
    pub root: Option<Entity>,
    pub overlay: Option<Entity>,
    pub children: BTreeMap<Entity, Vec<Entity>>,
    pub parent: BTreeMap<Entity, Option<Entity>>,
    iterator_start_node: Cell<Option<Entity>>,
}

impl Tree {
    /// Creates a new tree with default values.
    pub fn new() -> Self {
        Tree::default()
    }

    /// Returns the root of the tree.
    pub fn root(&self) -> Entity {
        if let Some(root) = self.root {
            return root;
        }

        if let Some(root) = self.parent.keys().next() {
            return *root;
        }

        0.into()
    }

    // /// Configure the tree iterator with a start node.
    pub fn start_node(&self, start_node: impl Into<Entity>) -> &Self {
        self.iterator_start_node.set(Some(start_node.into()));
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
        let root = root.into();
        self.root = Some(root);

        if let Some(overlay) = self.overlay {
            if let Some(p) = self.children.get_mut(&root) {
                p.push(overlay);
            }
        }
    }

    pub fn set_overlay(&mut self, overlay: impl Into<Entity>) {
        let overlay = overlay.into();
        self.overlay = Some(overlay);

        if let Some(root) = self.root {
            if let Some(p) = self.children.get_mut(&root) {
                p.push(overlay);
            }
        }
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

        if self.root.is_some() && parent == self.root.unwrap() && self.overlay.is_some() {
            // insert child to root before overlay
            let len = self.children.len();
            if let Some(p) = self.children.get_mut(&parent) {
                p.insert(len - 2, child);
            } else {
                return Err(NotFound::Parent(parent));
            }
        } else {
            if let Some(p) = self.children.get_mut(&parent) {
                p.push(child);
            } else {
                return Err(NotFound::Parent(parent));
            }
            self.parent.insert(child, Some(parent));
        }

        Ok(child)
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

impl EntityStore for Tree {
    fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.register_node(entity.into());
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();

        if let Some(parent_index) = self.parent[&entity] {
            if let Some(parent_children) = self.children.get_mut(&parent_index) {
                if let Some(index) = parent_children.iter().position(|&r| r == entity) {
                    parent_children.remove(index);
                }
            }
        }

        self.children.remove(&entity);
        self.parent.remove(&entity);
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = Entity;
    type IntoIter = TreeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let start_node = {
            if let Some(start_node) = self.iterator_start_node.get() {
                start_node
            } else if let Some(root) = self.root {
                root
            } else {
                0.into()
            }
        };

        self.iterator_start_node.set(None);

        TreeIterator {
            tree: self,
            start_node,
            current_node: None,
        }
    }
}

/// Used to create an iterator for the tree.
pub struct TreeIterator<'a> {
    tree: &'a Tree,
    start_node: Entity,
    current_node: Option<Entity>,
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        if let Some(node) = self.current_node {
            if !self.tree.children.contains_key(&node) {
                panic!("TreeIterator.next: Tree does not contains node {}", node.0);
            }
            if !self.tree.children[&node].is_empty() {
                self.current_node = Some(self.tree.children[&node][0]);
                return self.current_node;
            } else {
                if !self.tree.children.contains_key(&node) {
                    panic!("TreeIterator.next: Tree does not contains node {}", node.0);
                }
                let mut tree_node = node;
                while let Some(parent) = self.tree.parent[&tree_node] {
                    let siblings = &self.tree.children[&parent];

                    let sibling_index = siblings.iter().position(|&r| r == tree_node).unwrap() + 1;

                    if sibling_index < siblings.len() {
                        self.current_node = Some(siblings[sibling_index]);
                        return self.current_node;
                    } else {
                        tree_node = parent;
                    }
                }
                // root
                return None;
            }
        }

        self.current_node = Some(self.start_node);
        self.current_node
    }
}

#[cfg(test)]
mod tests {
    use dces::entity::EntityStore;
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
