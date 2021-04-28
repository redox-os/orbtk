use orbtk_core::*;

pub struct RenderContext2D {}

impl RenderContext2D {
    pub fn draw_text(
        &mut self,
        text: impl Into<String>,
        position: Point,
        font_size: u32,
        font_family: &str,
    ) {
    }
}
