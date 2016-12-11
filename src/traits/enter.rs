pub trait Enter {
    fn emit_enter(&self);
    fn on_enter<T: Fn(&Self) + 'static>(&self, func: T) -> &Self;
}
