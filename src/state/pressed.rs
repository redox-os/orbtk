use {Entity, EntityComponentManager, Selector, State, Tree};

pub struct PressedState;

impl State for PressedState {
    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
        fn update_selectors(entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
            if let Ok(selector) = ecm.borrow_mut_component::<Selector>(entity) {
                selector.pseudo_classes.insert(String::from("active"));
            }

            for child in &tree.children[&entity] {
                update_selectors(*child, tree, ecm);
            }
        }

        update_selectors(entity, tree, ecm);
    }
}

pub struct ReleaseState;

impl State for ReleaseState {
    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
        fn update_selectors(entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
            if let Ok(selector) = ecm.borrow_mut_component::<Selector>(entity) {
                selector.pseudo_classes.remove(&String::from("active"));
            }

            for child in &tree.children[&entity] {
                update_selectors(*child, tree, ecm);
            }
        }

        update_selectors(entity, tree, ecm);
    }
}