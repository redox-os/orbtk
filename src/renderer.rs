use super::{Color, Point, Rect};

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn char(&mut self, pos: Point, c: char, color: Color);
    fn pixel(&mut self, point: Point, color: Color);
    fn arc(&mut self, center: Point, radius: i32, parts: u8, color: Color);
    fn circle(&mut self, center: Point, radius: i32, color: Color);
    fn line(&mut self, start: Point, end: Point, color: Color);
    fn rect(&mut self, rect: Rect, color: Color);
}
