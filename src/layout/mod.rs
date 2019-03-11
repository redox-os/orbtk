use std::{cell::RefCell, collections::BTreeMap, rc::Rc, any::Any };

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
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
pub trait Layout: Any {
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
