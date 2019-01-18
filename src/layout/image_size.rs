use orbclient::Renderer;

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{Constraint, Image},
    theme::Theme,
    LayoutResult,
};

pub struct ImageSizeLayout;

impl Into<Box<dyn Layout>> for ImageSizeLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for ImageSizeLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        _constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Ok(image) = ecm.borrow_component::<Image>(entity) {
            return LayoutResult::Size((image.width(), image.height()));
        }

        LayoutResult::Size((0, 0))
    }
}
