// Styling related properties.

pub use self::background::{Background, BackgroundProperty};

mod background;
mod border_brush;
mod border_radius;
mod border_thickness;
mod font_size;
mod foreground;
mod icon_font;
mod icon_size;

#[cfg(test)]
mod tests;