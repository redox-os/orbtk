use crate::*;

pub trait RenderContext2D {
    fn draw_text(&mut self, text: &str, position: Point, font_size: u32, font_family: &str);

    /// Return the pixmap data lenght as an [u8] reference value.
    ///
    /// Byteorder: RGBA
    fn data(&self) -> &[u8];

    /// Return the pixmap data lenght as a mutable [u8] reference value.
    ///
    /// Byteorder: RGBA
    fn data_mut(&mut self) -> &mut [u8];

    /// Return the pixmap data lenght as a mutable [u8] reference value.
    ///
    /// Byteorder: RGBA
    fn data_u8_mut(&mut self) -> &mut [u8];
}
