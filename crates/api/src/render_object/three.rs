use crate::{prelude::*, utils::*};

pub struct ThreeRenderObject;

impl Into<Box<dyn RenderObject>> for ThreeRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ThreeRenderObject {
    fn render_self(&self, context: &mut Context<'_>, _: &Point) {
        // let background = context.widget().get::<Background>().0.clone();

        // context.render_context_2_d().Three(&background);
    }
}
