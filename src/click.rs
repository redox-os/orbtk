use super::Point;

pub trait Click {
    fn trigger_click(&self, point: Point);
    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self;
}
