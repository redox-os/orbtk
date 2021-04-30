use legion::world::Entry;

use crate::{components::*, *};

pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn draw(&self, entry: Entry, rtx: &mut dyn RenderContext2D) {
        // let mut render_query = <(&BoundsComponent, &FontComponent, &TextComponent)>::query();
        if let Ok(bounds) = entry.get_component::<BoundsComponent>() {
            if let Ok(font) = entry.get_component::<FontComponent>() {
                if let Ok(text) = entry.get_component::<TextComponent>() {
                    rtx.draw_text(
                        text.text.as_str(),
                        bounds.position,
                        font.size,
                        font.family.as_str(),
                    );
                }
            }
        }
    }
}
