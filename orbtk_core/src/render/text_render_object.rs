use legion::*;

use crate::{components::*, *};

pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn draw(&self, world: &World, rtx: &mut dyn RenderContext2D) {
        let mut render_query = <(&BoundsComponent, &FontComponent, &TextComponent)>::query();

        for (bounds, font, text) in render_query.iter(world) {
            rtx.draw_text(
                text.text.as_str(),
                bounds.position,
                font.size,
                font.family.as_str(),
            );
        }
    }
}
