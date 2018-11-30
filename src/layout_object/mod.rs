//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.

use dces::{Entity, EntityComponentManager};

use properties::Constraint;
use systems::LayoutResult;
use theme::Theme;

pub use self::center::CenterLayoutObject;
pub use self::flex::FlexLayoutObject;
pub use self::font_icon_size::FontIconSizeLayoutObject;
pub use self::padding::PaddingLayoutObject;
pub use self::root::RootLayoutObject;
pub use self::scroll::ScrollLayoutObject;
pub use self::stretch::StretchLayoutObject;
pub use self::text_size::TextSizeLayoutObject;

mod center;
mod flex;
mod font_icon_size;
mod padding;
mod root;
mod scroll;
mod stretch;
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
