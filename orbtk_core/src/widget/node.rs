use legion::*;

use crate::{components::*, *};

pub trait Component: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Component for T {}

#[derive(Debug)]
pub struct Node {
    pub entity: Entity,
    pub world: World,
    pub children: Option<Vec<Node>>,
}

impl Node {
    /// Creates a new node.
    pub fn new<T: 'static>() -> Self {
        let (world, entity) = Self::create::<T>();

        Self {
            entity,
            world,
            children: None,
        }
    }

    /// Pushes a new component to the node.
    pub fn push(&mut self, component: impl Component) {
        if let Some(mut entry) = self.world.entry(self.entity) {
            entry.add_component(component);
        }
    }

    pub fn from_children<T: 'static>(children: Vec<Node>) -> Self {
        let (world, entity) = Self::create::<T>();

        Self {
            entity,
            world,
            children: Some(children),
        }
    }

    fn create<T: 'static>() -> (World, Entity) {
        let mut world = World::default();

        // add default set of components to the node
        let entity = world.push((BoundsComponent::default(), TypeComponent::new::<T>()));

        (world, entity)
    }
}
