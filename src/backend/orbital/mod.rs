//! This module contains the concrete implementation of the OrbClient based backend.

use std::sync::Arc;

use orbfont::Font;

pub use self::backend::{WindowBuilder, WindowShell, ShellRunner, WindowAdapter};

use crate::backend::*;

mod backend;
mod renderer;
pub struct OrbFontMeasure;

impl FontMeasure for OrbFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        if font_size == 0 {
            return (0, 0);
        }
        let text = font.render(text, font_size as f32);
        (text.width(), text.height())
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<OrbFontMeasure> = { Arc::new(OrbFontMeasure) };
}
