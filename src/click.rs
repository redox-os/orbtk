use super::Point;

pub trait Click {
    fn click(&mut self, point: Point);
    fn on_click<T: Fn(&mut Self, Point) + 'static>(mut self, func: T) -> Self;
}
