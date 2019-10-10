// Widget related properties.
pub use self::clip::*;
pub use self::count::*;
pub use self::font_icon::*;
pub use self::image::*;
pub use self::name::*;
pub use self::resizeable::*;
pub use self::selected_entities::*;
pub use self::selected_indices::*;
pub use self::selection_mode::*;
pub use self::text::*;
pub use self::text_selection::*;
pub use self::title::*;
pub use self::water_mark::*;

mod clip;
mod count;
mod font_icon;
mod image;
mod name;
mod resizeable;
mod selected_entities;
mod selected_indices;
mod selection_mode;
mod text;
mod text_selection;
mod title;
mod water_mark;

#[cfg(test)]
mod tests;