pub trait Enter {
    fn emit_enter(&self);
    fn on_enter<T: Fn(&Self) + 'static>(mut self, func: T) -> Self;
}
