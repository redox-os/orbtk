pub use self::tree_builder::{Constraint, TreeBuilder};
pub use self::tree_renderer::TreeRenderer;

mod tree_builder;
mod tree_renderer;

use super::Widget;
use std::slice::Iter;
use std::sync::Arc;

pub type Id = usize;

#[derive(Default, Debug)]
pub struct Tree {
    pub graph: Graph,
    pub widgets: Vec<Arc<Widget>>,
}

impl Tree {
    pub fn widget(&self, id: Id) -> Arc<Widget> {
        self.widgets[id].clone()
    }

    pub fn print(&self) {
        println!("OrbTk tree");

        if  self.widgets.len() > 0 {
            self.print_tree(0, "");
        } else {
            println!("- Is empty");
        }
    }

      pub fn print_tree(&self, id: Id, spacer: &str) {
        println!("{}|- {}", spacer, self.widgets[id].element());
        let mut spacer = String::from(spacer);
        spacer.push_str("|    ");
        for child in self.graph.children(id) {
            self.print_tree(*child, &spacer);
        }
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    root: Id,
    children: Vec<Vec<Id>>,
    parent: Vec<Id>,
}

impl Graph {
    pub fn root(&self) -> Id {
        self.root
    }

    pub fn children(&self, parent: Id) -> Iter<Id> {
        self.children[parent].iter()
    }

    pub fn alloc_node(&mut self) -> Id {
        let id = self.children.len();
        self.children.push(vec![]);
        self.parent.push(id);
        id
    }

    pub fn append_child(&mut self, parent: Id, child: Id) {
        self.children[parent].push(child);
        self.parent[child] = parent;
    }
}
