use std::sync::Arc;

use super::{Id, Graph, Tree};
use widgets::{Content, Widget};

#[derive(Copy, Clone, Debug, Default)]
pub struct Constraint {
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
}

pub struct TreeBuilder;

impl TreeBuilder {
    pub fn new() -> Self {
        TreeBuilder {}
    }

    pub fn build(&self, widget: &Arc<Widget>) -> Tree {
        let mut graph = Default::default();
        let mut widgets = vec![];

        self.build_tree(&mut graph, &mut widgets, widget);

        Tree {
            graph,
            widgets,
        }
    }

    fn build_tree(&self, graph: &mut Graph, widgets: &mut Vec<Arc<Widget>>, widget: &Arc<Widget>) -> Id {   
        let node_id = graph.alloc_node();
        widgets.push(widget.clone());

        let mut widget_children = vec![];

        match widget.build() {
            Content::None => return node_id,
            Content::Single(child) => {
                widget_children.push(child);
            }
            Content::Multi(mut children) => {
                widget_children.append(&mut children);
            },
        }

        for child in widget_children {
            let child_id = self.build_tree(graph, widgets, &child);
            graph.append_child(node_id, child_id);
        }

        node_id
    }
}
