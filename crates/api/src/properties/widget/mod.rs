// Widget related properties.
pub use self::font_icon::*;
pub use self::image::*;
pub use self::name::*;
pub use self::resizeable::*;
pub use self::text::*;
pub use self::text_selection::*;
pub use self::title::*;
pub use self::water_mark::*;
pub use self::value::*;

mod font_icon;
mod image;
mod name;
mod resizeable;
mod text;
mod text_selection;
mod title;
mod water_mark;
mod value;

#[cfg(test)]
mod tests;
