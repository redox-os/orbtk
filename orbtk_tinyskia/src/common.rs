use crate::utils::*;
use std::f64::consts::{FRAC_PI_2, PI};

/// Calculates the AABB of a arc.
pub fn arc_rect(x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Rectangle {
    // Lazy calculate the AABB of its ends.
    let (a_y, a_x) = f64::sin_cos(start_angle);
    let (b_y, b_x) = f64::sin_cos(end_angle);
    let mut start_x = a_x.min(b_x);
    let mut start_y = a_y.min(b_y);
    let mut end_x = a_x.max(b_x);
    let mut end_y = a_y.max(b_y);
    // Moves in a circle through 90° steps, checking if that angle is part of the arc and
    // if that is true, then expanding the AABB to cover that angle position too.
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
                _ => break, // Error with the comparison in the while head?, no matters, break the loop.
            };
            start_x = start_x.min(x);
            start_y = start_y.min(y);
            end_x = end_x.max(x);
            end_y = end_y.max(y);
        }
        angle += f64::to_radians(90.0);
        i += 1;
    }
    // Maps the currently unit coords of the AABB to the position and radius of the arc.
    start_x = x + start_x * radius;
    start_y = y + start_y * radius;
    end_x = x + end_x * radius;
    end_y = y + end_y * radius;
    Rectangle::new((start_x, start_y), (end_x - start_x, end_y - start_y))
}

// Calculates the AABB of a quad bezier curve.
pub fn quad_bezier_rect(p0: Point, p1: Point, p2: Point) -> Rectangle {
    let mut mi = p0.min(p2);
    let mut ma = p0.max(p2);

    if p1.x() < mi.x() || p1.x() > ma.x() || p1.y() < mi.y() || p1.y() > ma.y() {
        // The control point is outside of the ends rectangle, possibly the AABB of the curve is larger
        let t = ((p0 - p1) / (p0 - 2.0 * p1 + p2)).clamp(0.0, 1.0);
        let s = Point::from(1.0) - t;
        let q = s * s * p0 + 2.0 * s * t * p1 + t * t * p2;
        mi = mi.min(q);
        ma = ma.max(q);
    }
    Rectangle::new(mi, Size::new(ma.x() - mi.x(), ma.y() - mi.y()))
}

// Calculates the AABB of a cubic bezier curve.
pub fn cubic_bezier_rect(p0: Point, p1: Point, p2: Point, p3: Point) -> Rectangle {
    let mut mi = p0.min(p3);
    let mut ma = p0.max(p3);

    let coefficient_c = -1.0 * p0 + 1.0 * p1;
    let coefficient_b = 1.0 * p0 - 2.0 * p1 + 1.0 * p2;
    let coefficient_a = -1.0 * p0 + 3.0 * p1 - 3.0 * p2 + 1.0 * p3;

    // let res = coefficient_b * coefficient_b - coefficient_a * coefficient_c;
    let mut res = coefficient_b * coefficient_b;
    res = res - coefficient_a * coefficient_c;
    if res.x() > 0.0 || res.y() > 0.0 {
        // The control point is outside of the ends rectangle, possibly the AABB of the curve is larger.
        let sqrt = res.abs().sqrt();
        let t1 = ((-coefficient_b - sqrt) / coefficient_a).clamp(0.0, 1.0);
        let t2 = ((-coefficient_b + sqrt) / coefficient_a).clamp(0.0, 1.0);
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

        if res.x() > 0.0 {
            mi.set_x(mi.x().min(q1.x().min(q2.x())));
            ma.set_x(ma.x().max(q1.x().max(q2.x())));
        }

        if res.y() > 0.0 {
            mi.set_y(mi.y().min(q1.y().min(q2.y())));
            ma.set_y(ma.y().max(q1.y().max(q2.y())));
        }
    }
    Rectangle::new(mi, Size::new(ma.x() - mi.x(), ma.y() - mi.y()))
}

/// A object used to keep a record of the AABB of a path.
#[derive(Debug, Copy, Clone)]
pub struct PathRect {
    clip_rect: Option<Rectangle>,
    path_rect: Option<Rectangle>,
    last_path_point: Point,
    first_path_point: Option<Point>,
}

impl PathRect {
    /// Creates a new `PathRect`, the clip parameter allows you to enclose it in
    /// a rectangle from the start, of course you can use `set_clip(false)` to release it after.
    pub fn new(clip: Option<Rectangle>) -> PathRect {
        PathRect {
            clip_rect: clip,
            path_rect: None,
            last_path_point: Point::new(0.0, 0.0),
            first_path_point: None,
        }
    }

    /// Records the closing of the path.
    pub fn record_path_close(&mut self) {
        self.last_path_point = self
            .first_path_point
            .unwrap_or_else(|| Point::new(0.0, 0.0));
    }

    /// Records the drawing of a rectangle.
    pub fn record_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
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

    /// Records the drawing of a arc
    pub fn record_arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
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

    /// Records the movement of the path drawing brush to a new location,
    /// the unique difference with `record_line_to` is that if the path has
    /// not started, It does not assume the drawing of a line from (0.0, 0.0).
    pub fn record_move_to(&mut self, x: f64, y: f64) {
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

    /// Records the drawing of a line
    pub fn record_line_to(&mut self, x: f64, y: f64) {
        if let Some(ref mut path_rect) = self.path_rect {
            path_rect.join_with_point(&Point::new(x, y));
        } else {
            self.path_rect = Some(Rectangle::new(Point::new(0.0, 0.0), (x, y)));
        }
        self.last_path_point = Point::new(x, y);
        if self.first_path_point.is_none() {
            self.first_path_point = Some(Point::new(0.0, 0.0));
        }
    }

    /// Records the drawing of a quadratic bezier curve.
    pub fn record_quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let r = quad_bezier_rect(self.last_path_point, Point::new(cpx, cpy), Point::new(x, y));
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

    /// Records the drawing of a cubic bezier curve.
    pub fn record_bezier_curve_to(
        &mut self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) {
        let r = cubic_bezier_rect(
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

    /// Encloses the current `PathRect`.
    pub fn record_clip(&mut self) {
        let mut clip_rect = match self.path_rect {
            Some(path_rect) => path_rect,
            None => return,
        };
        if let Some(current_clip_rect) = self.clip_rect {
            clip_rect.box_into(current_clip_rect);
        }
        self.clip_rect = Some(clip_rect);
    }

    #[cfg(all(
        not(target_arch = "wasm32"),
        any(feature = "default", feature = "orbraq"),
    ))]
    /// Checks if the current instance is enclosed.
    pub fn get_clip(&self) -> Option<Rectangle> {
        self.clip_rect
    }

    /// Gets the current path AABB, or nothing if the path is empty.
    pub fn get_rect(&self) -> Option<Rectangle> {
        self.path_rect.map(|mut r| {
            if let Some(clip) = self.clip_rect {
                r.box_into(clip);
            }
            r
        })
    }

    /// Restores itself to a new life of service, if the the path is clipped that state is conserved.
    pub fn rebirth(&mut self) {
        *self = Self::new(self.clip_rect);
    }
}

/// Resolves every gradient stop in `stops`, calling for each one the function `f`,
/// and giving it as parameters the position in a range from 0.0 to 1.0 and the
/// color of the stop.
pub fn build_unit_percent_gradient<F, R>(stops: &[GradientStop], length: f64, f: F) -> Vec<R>
where
    F: Fn(f64, Color) -> R,
{
    let mut r_stops = Vec::with_capacity(stops.len());
    let mut cursor = 0;
    let mut last_pos = 0.0;
    while cursor < stops.len() {
        if let Some(pos) = stops[cursor].pos {
            let pos = pos.unit_percent(length).min(1.0);
            let c = stops[cursor].color;
            r_stops.push(f(pos.max(last_pos), c));
            last_pos = pos;
            cursor += 1;
        } else {
            let mut second_cursor = cursor;
            let mut end = None;
            while second_cursor < stops.len() {
                if let Some(pos) = stops[second_cursor].pos {
                    end = Some(pos);
                    break;
                }
                second_cursor += 1;
            }
            let from_pos = match cursor == 0 {
                true => 0.0,
                false => match stops[cursor - 1].pos {
                    Some(p) => p.unit_percent(length),
                    None => unreachable!(),
                },
            };
            let mut count = (second_cursor - cursor) as f64;
            let to_pos = match end {
                Some(tp) => tp.unit_percent(length),
                None => {
                    count -= 1.0;
                    1.0
                }
            };
            for (i, stop) in stops.iter().enumerate().take(second_cursor).skip(cursor) {
                let p = (from_pos + (to_pos - from_pos) / count * (i as f64)).min(1.0);
                let c = stop.color;
                r_stops.push(f(p.max(last_pos), c));
                last_pos = p;
            }
            if end.is_none() {
                break;
            }
            cursor = second_cursor;
        }
    }
    r_stops
}

// Given an angle and a `Size` this function returns the ends of that gradient in the frame size gived
pub fn linear_gradient_ends_from_angle(angle: Angle, size: Size) -> Point {
    let angle = TAU - ((angle.to_radians() + FRAC_PI_2) % TAU);
    let a = size.width();
    let b = size.height();
    let c = (b / a).atan();
    let mut z;
    if angle > TAU - c || angle <= c || (angle > PI - c && angle < PI + c) {
        z = Point::new(a / 2.0, a / 2.0 * -angle.tan());
        if angle > FRAC_PI_2 && angle < PI + FRAC_PI_2 {
            z = -z;
        }
    } else {
        z = Point::new(b / (2.0 * -angle.tan()), b / 2.0);
        if angle < PI {
            z = -z;
        }
    }
    z
}

#[cfg(test)]
mod tests {
    use super::PathRect;

    #[test]
    fn test_pathrect_lines() {
        let mut rect = PathRect::new(None);
        rect.record_line_to(100.0, 20.0);
        rect.record_line_to(60.0, 10.0);
        let urect = rect.get_rect().unwrap();
        assert!(0.0 - urect.x().abs() < f64::EPSILON);
        assert!(0.0 - urect.y().abs() < f64::EPSILON);
        assert!(100.0 - urect.width().abs() < f64::EPSILON);
        assert!(20.0 - urect.height().abs() < f64::EPSILON);
        rect.rebirth();
        rect.record_move_to(30.0, 900.0);
        rect.record_line_to(-10.0, 859.0);
        rect.record_line_to(24.0, 800.0);
        let urect = rect.get_rect().unwrap();
        assert!(-10.0 - urect.x().abs() < f64::EPSILON);
        assert!(800.0 - urect.y().abs() < f64::EPSILON);
        assert!(40.0 - urect.width().abs() < f64::EPSILON);
        assert!(100.0 - urect.height().abs() < f64::EPSILON);
    }

    #[test]
    fn test_pathrect_arcs() {
        let mut rect = PathRect::new(None);
        rect.record_arc(40.0, 20.0, 5.0, 0.0, f64::to_radians(320.0));
        let urect = rect.get_rect().unwrap();
        assert!(35.0 - urect.x().abs() < f64::EPSILON);
        assert!(15.0 - urect.y().abs() < f64::EPSILON);
        assert!(10.0 - urect.width().abs() < f64::EPSILON);
        assert!(10.0 - urect.height().abs() < f64::EPSILON);
        rect.rebirth();
        rect.record_arc(40.0, 20.0, 5.0, 0.0, f64::to_radians(30.0));
        let urect = rect.get_rect().unwrap();
        assert!(44.33012701892219 - urect.x().abs() < f64::EPSILON);
        assert!(20.0 - urect.y().abs() < f64::EPSILON);
        assert!(0.669872981077809 - urect.width().abs() < f64::EPSILON);
        assert!(2.5 - urect.height().abs() < f64::EPSILON);
    }

    #[test]
    fn test_pathrect_rects() {
        let mut rect = PathRect::new(None);
        rect.record_rect(10.0, 10.0, 30.0, 30.0);
        rect.record_rect(49.0, 23.0, 60.0, 1.0);
        rect.record_rect(-1.0, -100.0, 20.0, 430.0);
        let urect = rect.get_rect().unwrap();
        assert!(-1.0 - urect.x().abs() < f64::EPSILON);
        assert!(10.0 - urect.y().abs() < f64::EPSILON);
        assert!(61.0 - urect.width().abs() < f64::EPSILON);
        assert!(420.0 - urect.height().abs() < f64::EPSILON);
    }

    #[test]
    fn test_pathrect_quad_beziers() {
        let mut rect = PathRect::new(None);
        rect.record_move_to(2.0, -1.0);
        rect.record_quadratic_curve_to(50.0, 6.0, 0.76, 30.0);
        let urect = rect.get_rect().unwrap();
        assert!(0.76 - urect.x().abs() < f64::EPSILON);
        assert!(-1.0 - urect.y().abs() < f64::EPSILON);
        assert!(24.933953105717812 - urect.width().abs() < f64::EPSILON);
        assert!(31.0 - urect.height().abs() < f64::EPSILON);
    }

    #[test]
    fn test_pathrect_cubic_beziers() {
        let mut rect = PathRect::new(None);
        rect.record_move_to(98.0, -100.0);
        rect.record_bezier_curve_to(20.0, 0.0, 15.0, 200.0, 0.0, 0.0);
        let urect = rect.get_rect().unwrap();
        assert!(0.0 - urect.x().abs() < f64::EPSILON);
        assert!(-100.0 - urect.y().abs() < f64::EPSILON);
        assert!(98.0 - urect.width().abs() < f64::EPSILON);
        assert!(185.57550765359252 - urect.height().abs() < f64::EPSILON);
    }
}
