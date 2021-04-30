use orbtk_core::{components::*, *};

#[derive(Default, Debug)]
pub struct Stack {
    children: Vec<Node>,
    orientation: OrientationComponent,
}

impl Stack {
    /// Creates a new stack layout widget.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method that is used to set the orientation of the stack layout widget..
    pub fn orientation(mut self, orientation: impl Into<OrientationComponent>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Builder method that is used to add a child to the stack layout widget.
    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
    }

    // todo align
}

impl Widget for Stack {
    fn build(self) -> Node {
        let mut node = Node::from_children::<Self>(self.children);
        node.push(self.orientation);

        node
    }
}
