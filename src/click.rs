use super::Point;

pub trait Click {
    fn click(&mut self, point: Point);
    fn on_click(mut self: Box<Self>, func: Box<Fn(&mut Self, Point)>) -> Box<Self>;
}
