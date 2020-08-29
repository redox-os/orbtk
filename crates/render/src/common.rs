use crate::utils::*;

pub fn arc_rect(x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Rectangle {
    let (a_y, a_x) = f64::sin_cos(start_angle);
    let (b_y, b_x) = f64::sin_cos(end_angle);
    let mut start_x = a_x.min(b_x);
    let mut start_y = a_y.min(b_y);
    let mut end_x = a_x.max(b_x);
    let mut end_y = a_y.max(b_y);
    let mut angle = 0.0;
    let min_angle = start_angle.min(end_angle);
    let max_angle = start_angle.max(end_angle);
    let mut i = 0;
    while angle < f64::to_radians(360.0) {
        if angle >= min_angle && angle <= max_angle {
            let (x, y) = match i {
                0 => (1.0, 0.0),
                1 => (0.0, 1.0),
                2 => (-1.0, 0.0),
                3 => (0.0, -1.0),
                _ => break,
            };
            start_x = start_x.min(x);
            start_y = start_y.min(y);
            end_x = end_x.max(x);
            end_y = end_y.max(y);
        }
        angle += f64::to_radians(90.0);
        i += 1;
    }
    start_x = x + start_x * radius;
    start_y = y + start_y * radius;
    end_x = x + end_x * radius;
    end_y = y + end_y * radius;
    Rectangle::new((start_x, start_y), (end_x - start_x, end_y - start_y))
}

pub fn quad_rect(p0: Point, p1: Point, p2: Point) -> Rectangle {
    let mut mi = p0.min(p2);
    let mut ma = p0.max(p2);

    if p1.x() < mi.x() || p1.x() > ma.x() || p1.y() < mi.y() || p1.y() > ma.y() {
        let t = ((p0 - p1) / (p0 - 2.0 * p1 + p2)).clamp(0.0, 1.0);
        let s = Point::from(1.0) - t;
        let q = s * s * p0 + 2.0 * s * t * p1 + t * t * p2;
        mi = mi.min(q);
        ma = ma.max(q);
    }
    Rectangle::new(mi, Size::new(ma.x() - mi.x(), ma.y() - mi.y()))
}

pub fn cubic_rect(p0: Point, p1: Point, p2: Point, p3: Point) -> Rectangle {
    let mut mi = p0.min(p3);
    let mut ma = p0.max(p3);

    let c = -1.0 * p0 + 1.0 * p1;
    let b = 1.0 * p0 - 2.0 * p1 + 1.0 * p2;
    let a = -1.0 * p0 + 3.0 * p1 - 3.0 * p2 + 1.0 * p3;

    let h = b * b - a * c;
    if h.x() > 0.0 || h.y() > 0.0 {
        let g = h.abs().sqrt();
        let t1 = ((-b - g) / a).clamp(0.0, 1.0);
        let t2 = ((-b + g) / a).clamp(0.0, 1.0);
        let s1 = Point::from(1.0) - t1;
        let s2 = Point::from(1.0) - t2;
        let q1 = s1 * s1 * s1 * p0
            + 3.0 * s1 * s1 * t1 * p1
            + 3.0 * s1 * t1 * t1 * p2
            + t1 * t1 * t1 * p3;
        let q2 = s2 * s2 * s2 * p0
            + 3.0 * s2 * s2 * t2 * p1
            + 3.0 * s2 * t2 * t2 * p2
            + t2 * t2 * t2 * p3;

        if h.x() > 0.0 {
            mi.set_x(mi.x().min(q1.x().min(q2.x())));
            ma.set_x(ma.x().max(q1.x().max(q2.x())));
        }

        if h.y() > 0.0 {
            mi.set_y(mi.y().min(q1.y().min(q2.y())));
            ma.set_y(ma.y().max(q1.y().max(q2.y())));
        }
    }
    Rectangle::new(mi, Size::new(ma.x() - mi.x(), ma.y() - mi.y()))
}

#[derive(Debug, Copy, Clone)]
pub struct PathRectTrack {
    path_rect: Option<Rectangle>,
    last_path_point: Point,
    first_path_point: Option<Point>,
    is_the_path_rect_fixed: bool,
}

impl PathRectTrack {
    pub fn new(clip: bool) -> PathRectTrack {
        PathRectTrack {
            path_rect: None,
            last_path_point: Point::new(0.0, 0.0),
            first_path_point: None,
            is_the_path_rect_fixed: clip,
        }
    }

    pub fn close_path(&mut self) {
        self.last_path_point = self.first_path_point.unwrap_or(Point::new(0.0, 0.0));
    }

    pub fn record_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if !self.is_the_path_rect_fixed {
            let r = Rectangle::new((x, y), (width, height));
            if let Some(ref mut path_rect) = self.path_rect {
                path_rect.join_with_rectangle(&r);
            } else {
                self.path_rect = Some(r);
            }
            self.last_path_point = Point::new(x, y);
            if self.first_path_point.is_none() {
                self.first_path_point = Some(Point::new(x, y));
            }
        }
    }

    pub fn record_arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        if !self.is_the_path_rect_fixed {
            let r = arc_rect(x, y, radius, start_angle, end_angle);
            if let Some(ref mut path_rect) = self.path_rect {
                path_rect.join_with_rectangle(&r);
            } else {
                self.path_rect = Some(r);
            }
            let (mut end_y, mut end_x) = f64::sin_cos(end_angle);
            end_x = x + end_x * radius;
            end_y = y + end_y * radius;
            self.last_path_point = Point::new(end_x, end_y);
            if self.first_path_point.is_none() {
                self.first_path_point = Some(Point::new(end_x, end_y));
            }
        }
    }

    pub fn record_point_at(&mut self, x: f64, y: f64) {
        if !self.is_the_path_rect_fixed {
            if let Some(ref mut path_rect) = self.path_rect {
                path_rect.join_with_point(&Point::new(x, y));
            } else {
                self.path_rect = Some(Rectangle::new(Point::new(x, y), (0.0, 0.0)));
            }
            self.last_path_point = Point::new(x, y);
            if self.first_path_point.is_none() {
                self.first_path_point = Some(Point::new(x, y));
            }
        }
    }

    pub fn record_quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        if !self.is_the_path_rect_fixed {
            let r = quad_rect(self.last_path_point, Point::new(cpx, cpy), Point::new(x, y));
            if let Some(ref mut path_rect) = self.path_rect {
                path_rect.join_with_rectangle(&r);
            } else {
                self.path_rect = Some(r);
            }
            self.last_path_point = Point::new(x, y);
            if self.first_path_point.is_none() {
                self.first_path_point = Some(Point::new(x, y));
            }
        }
    }

    pub fn record_bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        if !self.is_the_path_rect_fixed {
            let r = cubic_rect(
                self.last_path_point,
                Point::new(cp1x, cp1y),
                Point::new(cp2x, cp2y),
                Point::new(x, y),
            );
            if let Some(ref mut path_rect) = self.path_rect {
                path_rect.join_with_rectangle(&r);
            } else {
                self.path_rect = Some(r);
            }
            self.last_path_point = Point::new(x, y);
            if self.first_path_point.is_none() {
                self.first_path_point = Some(Point::new(x, y));
            }
        }
    }

    pub fn set_clip(&mut self, clip: bool) {
        self.is_the_path_rect_fixed = clip;
    }

    pub fn get_clip(&mut self) -> bool {
        self.is_the_path_rect_fixed
    }

    pub fn get_rect(&self) -> Option<Rectangle> {
        self.path_rect
    }
}
