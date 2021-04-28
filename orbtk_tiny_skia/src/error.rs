/// Represents tiny skia specific errors.
#[derive(Clone, Debug)]
pub enum Error {
    /// Describes that it was not possible to create a new tiny-skia display.
    CannotCreateTinySkiaDisplay,
    /// Cannot load a font.
    CannotLoadFont,
}
