//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.

use dces::{Entity, EntityComponentManager};

use crate::properties::Constraint;
use crate::systems::LayoutResult;
use crate::theme::Theme;

pub use self::center::CenterLayoutObject;
pub use self::fixed_size::FixedSizeLayoutObject;
pub use self::flex::FlexLayoutObject;
pub use self::font_icon_size::FontIconSizeLayoutObject;
pub use self::image_size::ImageSizeLayoutObject;
pub use self::padding::PaddingLayoutObject;
pub use self::root::RootLayoutObject;
pub use self::scroll::ScrollLayoutObject;
pub use self::stretch::StretchLayoutObject;
pub use self::text_selection_layout::TextSelectionLayoutObject;
pub use self::text_size::TextSizeLayoutObject;

mod center;
mod fixed_size;
mod flex;
mod font_icon_size;
mod image_size;
mod padding;
mod root;
mod scroll;
mod stretch;
mod text_selection_layout;
mod text_size;

pub trait LayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult;
}
