use dces::{Entity, EntityComponentManager, System};

pub struct LayoutSystem {}

impl System for LayoutSystem {
    fn run(&self, entities: &Vec<Entity>, _ecm: &mut EntityComponentManager) {
        for _entity in entities {}
        println!("Do layout");
    }
}
