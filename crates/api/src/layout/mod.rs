use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::*};

pub use self::fixed_size::*;
pub use self::grid::*;
pub use self::padding::*;
pub use self::scroll::*;
pub use self::stack::*;
pub use self::text_selection::*;

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
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &mut Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize;

    /// Arranges an sizes the children.
    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &mut Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64);
}
