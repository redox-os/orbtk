use orbtk_tiny_skia::FontLoader;

use crate::*;

pub trait Runner {
    fn run(&mut self, font_loader: &FontLoader) -> Result<bool, Error>;
}
