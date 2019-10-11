use std::any::Any;

use crate::{prelude::*, render};

#[derive(Clone, PartialEq)]
struct EmptyRenderPipeline;

impl render::RenderPipeline for EmptyRenderPipeline {
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn render::RenderPipeline> {
        Box::new(self.clone())
    }
    fn draw_pipeline(&self, image: &mut render::RenderTarget) {}
}

property!(
    /// Used to render a pipeline.
    RenderPipeline(Box<dyn render::RenderPipeline>)
);

impl Default for RenderPipeline {
    fn default() -> Self {
        RenderPipeline(Box::new(EmptyRenderPipeline))
    }
}
