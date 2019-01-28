//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    properties::{Constraint, GridColumn, HorizontalAlignment, Margin, VerticalAlignment},
    systems::LayoutResult,
};

// --- obsolete ---


//pub use self::padding::PaddingLayout;
//pub use self::scroll::ScrollLayout;
//pub use self::text_selection_layout::TextSelectionLayout;
//

//mod padding;
//mod scroll;
//mod text_selection_layout;

// todo: stack layout

// --- obsolete ---
pub use self::fixed_size::FixedSizeLayout;
pub use self::grid::GridLayout;

mod grid;
mod fixed_size;

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

// --- helpers ---

pub fn get_vertical_alignment(entity: Entity, ecm: &EntityComponentManager) -> VerticalAlignment {
    if let Ok(vertical_alignment) = ecm.borrow_component::<VerticalAlignment>(entity) {
        return *vertical_alignment;
    }

    VerticalAlignment::default()
}

pub fn get_horizontal_alignment(
    entity: Entity,
    ecm: &EntityComponentManager,
) -> HorizontalAlignment {
    if let Ok(horizontal_alignment) = ecm.borrow_component::<HorizontalAlignment>(entity) {
        return *horizontal_alignment;
    }

    HorizontalAlignment::default()
}

pub fn get_margin(entity: Entity, ecm: &EntityComponentManager) -> Margin {
    if let Ok(margin) = ecm.borrow_component::<Margin>(entity) {
        return *margin;
    }

    Margin::default()
}

// todo provide helpers for basic properties get_.. borrow_.. borrow_mut..

// --- helpers ---
