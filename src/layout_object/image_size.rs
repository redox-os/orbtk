use orbclient::Renderer;
use orbimage;

use backend::{FontMeasure, FONT_MEASURE};
use dces::{Entity, EntityComponentManager};
use layout_object::LayoutObject;
use properties::Constraint;
use theme::{Selector, Theme};

use {Label, LayoutResult};

pub struct ImageSizeLayoutObject;

impl Into<Box<LayoutObject>> for ImageSizeLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for ImageSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        _constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Ok(image) = ecm.borrow_component::<orbimage::Image>(entity) {
            return LayoutResult::Size((image.width(), image.height()));
        }

        LayoutResult::Size((0, 0))
    }
}
