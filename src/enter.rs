pub trait Enter {
    fn trigger_on_enter(&self);
    fn on_enter<T: Fn(&Self) + 'static>(mut self, func: T) -> Self;
}
