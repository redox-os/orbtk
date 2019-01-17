use crate::theme::Theme;

/// Is used to provides data from the `Backend` to the `StateSystem` and `PostLayoutStateSystem`.
pub struct StateContext<'a> {
    pub theme: &'a Theme,
}