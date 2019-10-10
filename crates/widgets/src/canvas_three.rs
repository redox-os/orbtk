use crate::prelude::*;
// CanvasThreeState
// updates the ThreeObject property

widget!(
    /// CanvasThree is used to render 3D graphics.
    CanvasThree {
        /// Sets or shares the three render pipeline. 
        pipeline: RenderPipeline,

        /// Sets or shares the css selector property. 
        selector: Selector
    }
);

impl Template for CanvasThree {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("CanvasThree").selector("canvas-three")
    }
}