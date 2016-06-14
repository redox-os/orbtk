use super::{Color, Point, Rect};

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn char(&mut self, pos: Point, c: char, color: Color);
    fn rect(&mut self, rect: Rect, color: Color);
    fn line(&mut self, start: Point, end: Point, color: Color);
}
