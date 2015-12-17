pub trait Enter {
    fn trigger_enter(&self) -> bool;
    fn on_enter<T: Fn(&Self) -> bool + 'static>(mut self, func: T) -> Self;
}
