use crate::{api::prelude::*, proc_macros::*};

widget!(
    /// Canvas is used to render 3D graphics.
    Canvas {
        /// Sets or shares the render pipeline.
        render_pipeline: DefaultRenderPipeline
    }
);

impl Template for Canvas {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Canvas").style("canvas-three")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        PipelineRenderObject.into()
    }
}
