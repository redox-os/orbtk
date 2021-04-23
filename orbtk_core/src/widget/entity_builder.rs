use legion::*;

pub trait Component: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Component for T {}

pub struct EntityBuilder<'a> {
    entity: Entity,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn push(&mut self, component: impl Component) {
        if let Some(mut entry) = self.world.entry(self.entity) {
            entry.add_component(component);
        }
    }
}
