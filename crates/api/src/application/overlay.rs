pub use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

use dces::prelude::*;

use orbtk_proc_macros::WidgetCtx;
use orbtk_theming::Selector;
use orbtk_utils::prelude::*;

use crate::{
    event::*,
    layout::{AbsoluteLayout, Layout},
    properties::*,
    widget,
    widget_base::*,
};

widget!(
    /// The `Overlay` is used to draw its children on the top of all other widgets in the tree.
    Overlay
);

impl Template for Overlay {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Overlay")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(AbsoluteLayout::new())
    }
}
