#[derive(Debug)]
pub enum OrientationComponent {
    Horizontal,
    Vertical,
}

impl Default for OrientationComponent {
    fn default() -> Self {
        OrientationComponent::Vertical
    }
}
