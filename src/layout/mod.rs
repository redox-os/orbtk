//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.




use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    properties::Constraint,
    systems::LayoutResult,
};

// --- obsolete ---

//pub use self::fixed_size::FixedSizeLayout;
//pub use self::padding::PaddingLayout;
//pub use self::scroll::ScrollLayout;
//pub use self::text_selection_layout::TextSelectionLayout;
//
//mod fixed_size;
//mod padding;
//mod scroll;
//mod text_selection_layout;

// todo: stack layout

// --- obsolete ---

pub use self::grid::GridLayout;

mod grid;

pub trait Layout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        parent_constraint: &Constraint,
        children: &[Entity],
        size: Option<(f64, f64)>,
    ) -> LayoutResult;
}
