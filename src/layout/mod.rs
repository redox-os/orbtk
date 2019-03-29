use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Constraint, HorizontalAlignment, Margin, Padding, VerticalAlignment, Visibility},
    structs::DirtySize,
    theme::Theme,
};

pub use self::fixed_size::FixedSizeLayout;
pub use self::grid::GridLayout;
pub use self::padding::PaddingLayout;
pub use self::scroll::ScrollLayout;
pub use self::stack::StackLayout;
pub use self::text_selection::TextSelectionLayout;

mod fixed_size;
mod grid;
mod padding;
mod scroll;
mod stack;
mod text_selection;

/// A layout is used to dynamic order the children of a widget.
pub trait Layout {
    // Measure all children before the arrangement.
    fn measure(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> DirtySize;

    /// Arranges an sizes the children.
    fn arrange(
        &self,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> (f64, f64);
}

// --- helpers ---

fn get_property<T>(entity: Entity, ecm: &EntityComponentManager) -> T
where
    T: Clone + Component + Default,
{
    ecm.borrow_component::<T>(entity)
        .map(|r| r.clone())
        .unwrap_or_default()
}

pub fn get_vertical_alignment(entity: Entity, ecm: &EntityComponentManager) -> VerticalAlignment {
    get_property::<VerticalAlignment>(entity, ecm)
}

pub fn get_horizontal_alignment(
    entity: Entity,
    ecm: &EntityComponentManager,
) -> HorizontalAlignment {
    get_property::<HorizontalAlignment>(entity, ecm)
}

pub fn get_margin(entity: Entity, ecm: &EntityComponentManager) -> Margin {
    get_property::<Margin>(entity, ecm)
}

pub fn get_padding(entity: Entity, ecm: &EntityComponentManager) -> Padding {
    get_property::<Padding>(entity, ecm)
}

pub fn get_constraint(entity: Entity, ecm: &EntityComponentManager) -> Constraint {
    get_property::<Constraint>(entity, ecm)
}

pub fn get_visibility(entity: Entity, ecm: &EntityComponentManager) -> Visibility {
    get_property::<Visibility>(entity, ecm)
}

// todo provide helpers for basic properties get_.. borrow_.. borrow_mut..

// --- helpers ---
