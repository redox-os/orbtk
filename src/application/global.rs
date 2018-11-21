
use dces::Entity;

#[derive(Default)]
pub struct Global {
    pub focused_entity: Option<Entity>,
    pub mouse_over_entity: Option<Entity>,
}