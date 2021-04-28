use legion::World;

use crate::*;

/// Wrapper component for a render object.
pub struct RenderComponent {
    render_object: Box<dyn RenderObject>,
}

impl RenderComponent {
    pub fn new(render_object: impl RenderObject + 'static) -> Self {
        Self {
            render_object: Box::new(render_object),
        }
    }

    pub fn draw(&self, world: &World, rtx: &mut dyn RenderContext2D) {
        self.render_object.draw(world, rtx);
    }
}
