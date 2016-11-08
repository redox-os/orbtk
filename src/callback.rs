use super::Point;

pub trait Click {
    fn emit_click(&self, point: Point);
    fn on_click<T: Fn(&Self, Point) + 'static>(self, func: T) -> Self;
}

pub trait Enter {
    fn emit_enter(&self);
    fn on_enter<T: Fn(&Self) + 'static>(self, func: T) -> Self;
}
