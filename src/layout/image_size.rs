use orbclient::Renderer;

use crate::core::{ImageElement, Size};

use dces::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::Constraint,
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
        if let Ok(image) = ecm.borrow_component::<ImageElement>(entity) {
            return LayoutResult::Size((image.width() as u32, image.height() as u32));
        }

        LayoutResult::Size((0, 0))
    }
}
