pub trait Resize {
    fn emit_resize(&self, width: u32, height: u32);
    fn on_resize<T: Fn(&Self, u32, u32) + 'static>(&self, func: T) -> &Self;
}
