
use point::Point;

pub trait Click {
    fn emit_click(&self, point: Point);
    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self;
}
