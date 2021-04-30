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

    /// Creates a new entity with default set of components and returns an entity builder.
    pub fn create_entity<T: 'static>(&mut self) -> EntityBuilder {
        // add (render) bounds to the widget
        let entity = self.world.push((BoundsComponent::default(),));

        // creates tree from the root entity (first created entity).
        if self.tree.is_none() {
            self.tree = Some(Tree::new(entity));
        }

        let mut builder = EntityBuilder {
            entity,
            // unwrap because if there is no tree it will be created first.
            tree: self.tree.as_mut().unwrap(),
            world: &mut self.world,
        };

        // add type information to the widget entity
        builder.push(TypeComponent::new::<T>());

        builder
    }

    /// Consumes the inner of the build context.
    pub(crate) fn consume(self) -> (World, Resources) {
        (self.world, self.resources)
    }
}
