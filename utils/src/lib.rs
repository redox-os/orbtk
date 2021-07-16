pub use self::alignment::*;
pub use self::angle::*;
pub use self::border::*;
pub use self::brush::*;
pub use self::color::*;
pub use self::constraint::*;
pub use self::dirty_size::*;
pub use self::expression::*;
pub use self::f32_cmp::*;
pub use self::f64_cmp::*;
pub use self::filter::*;
pub use self::gradients::*;
pub use self::number::*;
pub use self::orientation::*;
pub use self::point::*;
pub use self::rectangle::*;
pub use self::relative_direction::*;
pub use self::selection_mode::*;
pub use self::size::*;
pub use self::string16::*;
pub use self::text_alignment::*;
pub use self::text_baseline::*;
pub use self::thickness::*;
pub use self::value::*;
pub use self::visibility::*;

mod alignment;
mod angle;
mod border;
mod brush;
mod color;
mod constraint;
mod dirty_size;
mod expression;
mod f32_cmp;
mod f64_cmp;
mod filter;
mod gradients;
mod number;
mod orientation;
mod point;
pub mod prelude;
mod rectangle;
mod relative_direction;
mod selection_mode;
mod size;
mod spacer;
mod string16;
mod text_alignment;
mod text_baseline;
mod thickness;
mod value;
mod visibility;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisualRenderProperties {
    pub bounds: Rectangle,
    pub background: Brush,
    pub border_radius: f64,
    pub border_thickness: Thickness,
    pub border_brush: Brush,
    pub visibility: Visibility,
}
