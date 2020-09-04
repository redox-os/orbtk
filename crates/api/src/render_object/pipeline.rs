use crate::{proc_macros::IntoRenderObject, render_object::*};

#[derive(Debug, IntoRenderObject)]
pub struct PipelineRenderObject;

impl RenderObject for PipelineRenderObject {
    fn render_self(&self, ctx: &mut Context, _: &Point) {
        let bounds = *ctx.widget().get::<Rectangle>("bounds");
        let pipeline = ctx
            .widget()
            .get::<DefaultRenderPipeline>("render_pipeline")
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
