use legion::*;

use crate::{components::*, widget::*};

#[derive(Default)]
pub struct BuildContext {
    world: World,
    resources: Resources,
}

impl BuildContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.world.push((BoundsComponent::default(),));
        EntityBuilder {
            entity,
            world: &mut self.world,
        }
    }

    pub(crate) fn consume(self) -> (World, Resources) {
        (self.world, self.resources)
    }
}
