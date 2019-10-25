use crate::{prelude::*, utils::*};

pub struct PipelineRenderObject;

impl Into<Box<dyn RenderObject>> for PipelineRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for PipelineRenderObject {
    fn render_self(&self, context: &mut Context<'_>, _: &Point) {
        let bounds = context.widget().get::<Bounds>("bounds").0;
        let pipeline = context
            .widget()
            .get::<RenderPipeline>("render_pipeline")
            .0
            .clone();

        context.render_context_2_d().draw_pipeline(
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height,
            pipeline,
        );
    }
}
