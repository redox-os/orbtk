use crate::Value;

/// Used to describes a thickness e.g a border thickness.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Thickness {
    /// Left of thickness.
    pub left: f64,

    /// Top of thickness.
    pub top: f64,

    /// Right of thickness.
    pub right: f64,

    /// Bottom of thickness.
    pub bottom: f64,
}

impl Thickness {
    /// Create a new thickness with the given parameters.
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Gets left.
    pub fn left(&self) -> f64 {
        self.left
    }

    /// Sets left.
    pub fn set_left(&mut self, left: f64) {
        self.left = left;
    }

    /// Gets top.
    pub fn top(&self) -> f64 {
        self.top
    }

    /// Sets top.
    pub fn set_top(&mut self, top: f64) {
        self.top = top;
    }

    /// Gets right.
    pub fn right(&self) -> f64 {
        self.right
    }

    /// Sets right.
    pub fn set_right(&mut self, right: f64) {
        self.right = right;
    }

    /// Gets bottom.
    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    /// Sets bottom.
    pub fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }

    /// Gets thickness.
    pub fn thickness(&self) -> Thickness {
        *self
    }

    /// Sets thickness.
    pub fn set_thickness<T: Into<Thickness>>(&mut self, thickness: T) {
        let other = thickness.into();

        self.set_left(other.left());
        self.set_top(other.top());
        self.set_right(other.right());
        self.set_bottom(other.bottom());
    }
}

// --- Trait implementations ---

impl From<(i32, i32, i32, i32)> for Thickness {
    fn from(t: (i32, i32, i32, i32)) -> Self {
        Thickness::new(t.0 as f64, t.1 as f64, t.2 as f64, t.3 as f64)
    }
}

impl From<(i32, i32)> for Thickness {
    fn from(t: (i32, i32)) -> Self {
        Thickness::new(t.0 as f64, t.1 as f64, t.0 as f64, t.1 as f64)
    }
}

impl From<i32> for Thickness {
    fn from(t: i32) -> Self {
        Thickness::new(t as f64, t as f64, t as f64, t as f64)
    }
}

impl From<(f64, f64, f64, f64)> for Thickness {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Thickness::new(t.0, t.1, t.2, t.3)
    }
}

impl From<(f64, f64)> for Thickness {
    fn from(t: (f64, f64)) -> Self {
        Thickness::new(t.0, t.1, t.0, t.1)
    }
}

impl From<f64> for Thickness {
    fn from(t: f64) -> Self {
        Thickness::new(t, t, t, t)
    }
}

impl From<Value> for Thickness {
    fn from(v: Value) -> Self {
        match v.0 {
            ron::Value::Number(value) => Thickness::from(value.into_f64()),
            ron::Value::Map(map) => {
                let mut left = 0.0;
                let mut top = 0.0;
                let mut right = 0.0;
                let mut bottom = 0.0;

                for (key, value) in map.iter() {
                    if let Ok(key) = key.clone().into_rust::<String>() {
                        let value = if let Ok(value) = value.clone().into_rust::<f64>() {
                            value
                        } else {
                            0.0
                        };

                        if key.as_str().eq("left") {
                            left = value;
                        }

                        if key.as_str().eq("top") {
                            top = value;
                        }

                        if key.as_str().eq("right") {
                            right = value;
                        }

                        if key.as_str().eq("bottom") {
                            bottom = value;
                        }
                    }
                }

                Thickness::from((left, top, right, bottom))
            }
            _ => Thickness::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let rect = Thickness::new(5.0, 10.0, 20.0, 30.0);

        crate::f64_assert(rect.left, 5.0);
        crate::f64_assert(rect.top, 10.0);
        crate::f64_assert(rect.right, 20.0);
        crate::f64_assert(rect.bottom, 30.0);
    }

    #[test]
    fn test_into() {
        let thickness: Thickness = (10.0, 12.0, 13.0, 14.0).into();

        crate::f64_assert(thickness.left, 10.0);
        crate::f64_assert(thickness.top, 12.0);
        crate::f64_assert(thickness.right, 13.0);
        crate::f64_assert(thickness.bottom, 14.0);

        let thickness: Thickness = 10.0.into();

        crate::f64_assert(thickness.left, 10.0);
        crate::f64_assert(thickness.top, 10.0);
        crate::f64_assert(thickness.right, 10.0);
        crate::f64_assert(thickness.bottom, 10.0);
    }
}
