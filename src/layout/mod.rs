//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.

use dces::prelude::{Entity, EntityComponentManager};

use crate::properties::Constraint;
use crate::systems::LayoutResult;
use crate::theme::Theme;

pub use self::center::CenterLayout;
pub use self::fixed_size::FixedSizeLayout;
pub use self::flex::FlexLayout;
pub use self::font_icon_size::FontIconSizeLayout;
pub use self::image_size::ImageSizeLayout;
pub use self::padding::PaddingLayout;
pub use self::root::RootLayout;
pub use self::scroll::ScrollLayout;
pub use self::stretch::StretchLayout;
pub use self::text_selection_layout::TextSelectionLayout;
pub use self::text_size::TextSizeLayout;

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

pub trait Layout {
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
