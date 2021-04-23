use legion::*;

use crate::{components::*, widget::*};

pub struct BuildContext<'a> {
    world: &'a mut World,
}

impl<'a> BuildContext<'a> {
    pub fn new(world: &'a mut World) -> Self {
        BuildContext { world }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.world.push((BoundsComponent::default(),));
        EntityBuilder {
            entity,
            world: &mut self.world,
        }
    }
}
