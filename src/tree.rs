use std::cell::RefCell;
use std::collections::BTreeMap;

use {Entity, NotFound};

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
}

impl<'a> IntoIterator for &'a Tree {
    type Item = Entity;
    type IntoIter = TreeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIntoIterator {
            tree: self,
            path: RefCell::new(vec![]),
        }
    }
}

pub struct TreeIntoIterator<'a> {
    tree: &'a Tree,
    path: RefCell<Vec<Entity>>,
}

impl<'a> Iterator for TreeIntoIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let mut path = self.path.borrow_mut();
        let mut result = None;

        if path.len() == 0 {
            result = Some(self.tree.root);
        } else {
            let mut current_node = path[path.len() - 1];

            // if current node has children return the first child
            if self.tree.children.get(&current_node).unwrap().len() > 0 {
                result = Some(self.tree.children.get(&current_node).unwrap()[0]);
            } else {
                // if the node doesn't have kids check its siblings
                loop {
                    path.pop();

                    if path.len() == 0 {
                        break;
                    }

                    let parent = self.tree.parent.get(&current_node).unwrap();
                    let siblings = self.tree.children.get(parent).unwrap();
                    let sibling_index =
                        siblings.iter().position(|&r| r == current_node).unwrap() + 1;

                    if sibling_index < siblings.len() {
                        result = Some(siblings[sibling_index]);
                        break;
                    } else {
                        current_node = *parent;
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
