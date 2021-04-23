use legion::*;

use crate::widget::*;

pub struct BuildContext<'a> {
    world: &'a mut World,
}

impl<'a> BuildContext<'a> {
    pub fn new(world: &'a mut World) -> Self {
        BuildContext { world }
    }

    // pub fn create_entity(&mut self) -> EntityBuilder {
    //     // let entity = self.world.push((BoundsComponent {},));
    //     EntityBuilder {
    //         entity,
    //         world: &mut self.world,
    //     }
    // }
}
