use crate::prelude::*;

pub struct DefaultRenderObject;

impl Into<Box<dyn RenderObject>> for DefaultRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for DefaultRenderObject {}
