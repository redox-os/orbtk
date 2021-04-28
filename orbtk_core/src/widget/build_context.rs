use legion::*;

use crate::{components::*, widget::*};

/// `BuildContext` is internal used to create an entity with components from a widget.
#[derive(Default)]
pub struct BuildContext {
    world: World,
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
        let mut builder = EntityBuilder {
            entity,
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
