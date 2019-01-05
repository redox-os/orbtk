use std::collections::HashMap;

use dces::Entity;

use event::EventQueue;

use theme::Theme;

use orbrender::traits::Window;

#[derive(Default)]
/// The `Global` struct is used to define global `properties` that could be accsed application width.
pub struct Global {
    /// Contains the current focues widget.
    pub focused_widget: Option<Entity>,

    /// Used to reference widgets by its css id.
    pub id_map: HashMap<String, Entity>,

    pub theme: Theme,

    pub window: Option<Box<Window>>,

    pub event_queue: EventQueue,
}