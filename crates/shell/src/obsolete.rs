// !!! The complete components of this module will be deleted after OrbGL supports text rendering !!!

use orbfont::Font;

use orbtk_utils::prelude::*;

pub trait Renderer {
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &Font,
    );
}

pub trait FontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32);
}