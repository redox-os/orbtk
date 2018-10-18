use {Entity, EntityComponentManager, Property, EventBox, Tree};

pub trait State {
    fn handles_event(
        &self,
        _event: &EventBox,
        _entity: Entity,
        _ecm: &mut EntityComponentManager,
    ) -> bool {
        false
    }

    fn update(
        &self,
        _event: &EventBox,
        _entity: Entity,
        _tree: &Tree,
        _ecm: &mut EntityComponentManager,
    ) -> bool {
        false
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}
