use crate::{prelude::*, utils::*};

pub struct PipelineRenderObject;

impl Into<Box<dyn RenderObject>> for PipelineRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for PipelineRenderObject {
    fn render_self(&self, ctx: &mut Context, _: &Point) {
        let bounds = *ctx.widget().get::<Rectangle>("bounds");
        let pipeline = ctx
            .widget()
            .get::<RenderPipeline>("render_pipeline")
            .0
            .clone();

        ctx.render_context_2_d().draw_pipeline(
            bounds.x(),
            bounds.y(),
            bounds.width(),
            bounds.height(),
            pipeline,
        );
    }
}
