use std::cell::RefCell;
use std::collections::BTreeMap;

use {Entity, NotFound, EntityContainer};

#[derive(Default)]
pub struct Tree {
    pub root: Entity,
    pub children: BTreeMap<Entity, Vec<Entity>>,
    pub parent: BTreeMap<Entity, Entity>,
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

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl EntityContainer for Tree {
    fn register_entity(&mut self, entity: Entity) {
        self.register_node(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.children.remove(&entity);
        self.parent.remove(&entity);
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = Entity;
    type IntoIter = TreeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIterator {
            tree: self,
            path: RefCell::new(vec![]),
        }
    }
}

pub struct TreeIterator<'a> {
    tree: &'a Tree,
    path: RefCell<Vec<Entity>>,
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let mut path = self.path.borrow_mut();
        let mut result = None;

        if path.is_empty() {
            result = Some(self.tree.root);
        } else {
            let mut current_node = path[path.len() - 1];

            // if current node has children return the first child
            if ! self.tree.children[&current_node].is_empty() {
                result = Some(self.tree.children[&current_node][0]);
            } else {
                // if the node doesn't have kids check its siblings
                loop {
                    path.pop();

                    if path.is_empty() {
                        break;
                    }

                    let parent = self.tree.parent[&current_node];
                    let siblings = &self.tree.children[&parent];
                    let sibling_index =
                        siblings.iter().position(|&r| r == current_node).unwrap() + 1;

                    if sibling_index < siblings.len() {
                        result = Some(siblings[sibling_index]);
                        break;
                    } else {
                        current_node = parent;
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
