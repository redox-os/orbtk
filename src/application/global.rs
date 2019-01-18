use std::collections::HashMap;

use dces::prelude::Entity;

#[derive(Default)]
/// The `Global` struct is used to define global `properties` that could be accsed application width.
pub struct Global {
    /// Contains the current focues widget.
    pub focused_widget: Option<Entity>,

    /// Used to reference widgets by its css id.
    pub id_map: HashMap<String, Entity>,
}