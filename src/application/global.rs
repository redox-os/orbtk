use dces::Entity;

#[derive(Default)]
/// The `Global` struct is used to define global `properties` that could be accsed application width.
pub struct Global {
    /// Contains the current focues widget.
    pub focused_widget: Option<Entity>,
}