use legion::*;

use crate::{components::*, *};

/// `BuildContext` is internal used to create an entity with components from a widget.
#[derive(Default)]
pub struct BuildContext {
    world: World,
    tree: Option<Tree>,
    resources: Resources,
}

impl BuildContext {
    /// Creates a new builder context.
    pub fn new() -> Self {
        Self::default()
    }
}
