use crate::*;

pub trait RenderContext2D {
    fn draw_text(&mut self, text: &str, position: Point, font_size: u32, font_family: &str);
}
