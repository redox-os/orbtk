pub trait Border {
    fn border(&self, enabled: bool) -> &Self;
    fn border_radius(&self, radius: u32) -> &Self;
}
