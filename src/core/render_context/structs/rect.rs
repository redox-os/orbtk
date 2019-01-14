#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect {
            x,
            y, 
            width,
            height,
        }
    }
}

pub trait Size {
    fn set_with(&mut self, width: f64);
    fn width(&self) -> f64;
    fn set_height(&mut self, height: f64);
    fn height(&self) -> f64;
    fn set_size(&mut self, width: f64, height: f64);
    fn size(&self) -> (f64, f64);
}

pub trait Position {
    fn set_x(&mut self, x: f64);
    fn x(&self) -> f64;
    fn set_y(&mut self, y: f64);
    fn y(&self) -> f64;
    fn set_position(&mut self, x: f64, y: f64);
    fn position(&self) -> (f64, f64);
}
