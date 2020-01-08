use crate::prelude::*;

widget!(
    /// The `Overlay` is used to draw its children on the top of all other widgets in the tree.
    Overlay
);

impl Template for Overlay {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Overlay")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(AbsoluteLayout::new())
    }
}
