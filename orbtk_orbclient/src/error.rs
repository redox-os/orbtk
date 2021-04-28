/// Represents orbtk OrbClient specific errors.
#[derive(Clone, Debug)]
pub enum Error {
    /// Describes that it was not possible to create a new OrbClient window.
    CannotCreateWindow,
    /// Describes that it was not possible to read the screen size.
    CannotReadScreenSize,
    /// The source frame buffer has the wrong size.
    WrongFrameBufferSize,
    /// Cannot load a font.
    CannotLoadFont,
}
