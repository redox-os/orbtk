use std::any::Any;

use crate::render;

#[derive(Clone, PartialEq)]
struct EmptyRenderPipeline;

impl render::Pipeline for EmptyRenderPipeline {
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn render::Pipeline> {
        Box::new(self.clone())
    }
}

impl render::RenderPipeline for EmptyRenderPipeline {
    fn draw(&self, _: &mut render::RenderTarget) {}
}

/// RenderPipeline object.
#[derive(Clone, Debug)]
pub struct RenderPipeline(pub Box<dyn render::Pipeline>);

impl Default for RenderPipeline {
    fn default() -> Self {
        RenderPipeline(Box::new(EmptyRenderPipeline))
    }
}
