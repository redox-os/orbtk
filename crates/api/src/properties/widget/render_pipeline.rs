use std::any::Any;

use crate::render;

#[derive(Clone, PartialEq)]
struct EmptyRenderPipeline;

impl render::PipelineTrait for EmptyRenderPipeline {
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn render::PipelineTrait> {
        Box::new(self.clone())
    }
}

impl render::RenderPipeline for EmptyRenderPipeline {
    fn draw(&self, _: &mut render::RenderTarget) {}
}

/// RenderPipeline object.
#[derive(Clone, Debug)]
pub struct DefaultRenderPipeline(pub Box<dyn render::PipelineTrait>);

impl PartialEq for DefaultRenderPipeline {
    fn eq(&self, _other: &Self) -> bool {
        // todo this is workaround for property checking
        false
    }
}

impl Default for DefaultRenderPipeline {
    fn default() -> Self {
        DefaultRenderPipeline(Box::new(EmptyRenderPipeline))
    }
}
