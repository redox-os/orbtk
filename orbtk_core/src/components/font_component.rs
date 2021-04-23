#[derive(Clone, Default, Debug)]
pub struct FontComponent {
    pub family: String,
    pub size: u32,
}

impl FontComponent {
    pub fn new(family: impl Into<String>, size: u32) -> Self {
        Self {
            family: family.into(),
            size,
        }
    }
}
