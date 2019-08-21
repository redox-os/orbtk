use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{prelude::*, utils::*, render::RenderContext2D};

pub struct DefaultRenderObject;

impl Into<Box<dyn RenderObject>> for DefaultRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for DefaultRenderObject {
}
