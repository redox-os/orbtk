use crate::theme::Theme;

/// Is used to provides data from the `Backend` to the `LayoutSystem`.
pub struct LayoutContext<'a> {
    pub window_size: (u32, u32),
    pub theme: &'a Theme,
}