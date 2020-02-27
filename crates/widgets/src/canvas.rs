use crate::prelude::*;
// CanvasState
// updates the ThreeObject property

widget!(
    /// Canvas is used to render 3D graphics.
    Canvas {
        /// Sets or shares the three render pipeline.
        render_pipeline: RenderPipeline
    }
);

impl Template for Canvas {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Canvas").element("canvas-three")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(PipelineRenderObject)
    }
}
