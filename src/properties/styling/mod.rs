// Styling related properties.

pub use self::background::*;
pub use self::border_brush::*;
pub use self::border_radius::*;
pub use self::border_thickness::*;
pub use self::font_size::*;
pub use self::foreground::*;
pub use self::icon_brush::*;
pub use self::icon_brush::*;
pub use self::icon_size::*;
pub use self::opacity::*;

mod background;
mod border_brush;
mod border_radius;
mod border_thickness;
mod font_size;
mod foreground;
mod icon_brush;
mod icon_font;
mod icon_size;
mod opacity;

#[cfg(test)]
mod tests;