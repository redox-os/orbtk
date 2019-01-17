
use orbgl::prelude::Canvas;
use crate::theme::Theme;

pub struct RenderContext<'a> {
    pub renderer: &'a mut Canvas,
    pub theme: &'a Theme,
}