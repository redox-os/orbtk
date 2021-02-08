/// Represents morph_ui OrbClient specific errors.
#[derive(Clone, Debug)]
pub enum Error {
    /// Describes that it was not possible to create a new OrbClient window.
    CannotCreateWindow,
    /// Describes that it was not possible to read the screen size.
    CannotReadScreenSize,
}
