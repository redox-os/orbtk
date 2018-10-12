use {Entity, EntityComponentManager, Tree};

pub use self::pressed::*;

mod pressed;

pub trait State {
    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager);
}
